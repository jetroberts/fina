use std::{error::Error, time::Instant};

mod models;
mod services;

pub mod trans {
    tonic::include_proto!("transaction");
}

use trans::GetRequest;

use crate::trans::{transactor_client::TransactorClient, AddRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TransactorClient::connect("http://[127.0.0.1]:50051").await?;

    let request = tonic::Request::new(GetRequest {
        id: "1".to_string(),
    });

    let new_transaction = tonic::Request::new(AddRequest {
        amount: 100.0,
        transaction_type: "debit".to_string(),
        timestamp: 123.to_string(),
        user: "josh".to_string(),
    });

    let now = Instant::now();

    let _res = client.add(new_transaction).await?;
    println!("{:?}", _res);
    let _res = client.get(request).await?;

    println!("TIME={:?}", now.elapsed());
    Ok(())
}
