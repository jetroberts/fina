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
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        self.service.get_transaction()?;
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        todo!();
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
