use std::error::Error;

mod models;
mod services;

pub mod t {
    tonic::include_proto!("transaction");
}

use t::transactor_server::Transactor;
use t::{GetRequest, GetResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::t::transactor_server::TransactorServer;

#[derive(Default)]
struct TransactionService;

#[tonic::async_trait]
impl Transactor for TransactionService {
    async fn get_transaction(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        println!("Got response {:?}", request);

        let response = GetResponse {
            id: "1".to_string(),
            amount: 100.0,
            transaction_type: "debit".to_string(),
            user: "1".to_string(),
            category: "home".to_string(),
            timestamp: 1,
            created_at: 1,
            updated_at: 1,
            deleted_at: 1,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let transaction_server = TransactionService::default();

    println!("Transaction server listening on: {}", addr);

    Server::builder()
        .add_service(TransactorServer::new(transaction_server))
        .serve(addr)
        .await?;

    Ok(())
}
