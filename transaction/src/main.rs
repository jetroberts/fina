use std::error::Error;

mod database;
mod models;
mod services;

pub mod t {
    tonic::include_proto!("transaction");
}

use database::redis::Redis;
use services::transaction_service::TransactionService;
use t::transactor_server::Transactor;
use t::{AddRequest, AddResponse, GetRequest, GetResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::t::transactor_server::TransactorServer;

struct TransactionController {
    service: TransactionService<Redis>,
}

impl TransactionController {
    fn new() -> Self {
        Self {
            service: TransactionService::new(Redis::new()),
        }
    }
}

#[tonic::async_trait]
impl Transactor for TransactionController {
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let res = match self.service.get_transaction() {
            Ok(r) => r,
            Err(e) => return Err(Status::new(tonic::Code::Internal, e.to_string())),
        };

        Ok(Response::new(res))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let res = match self.service.add_transaction(request.into_inner()) {
            Ok(r) => r,
            Err(e) => return Err(Status::new(tonic::Code::Internal, e.to_string())),
        };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let transaction_server = TransactionController::new();

    println!("Transaction server listening on: {}", addr);

    Server::builder()
        .add_service(TransactorServer::new(transaction_server))
        .serve(addr)
        .await?;

    Ok(())
}
