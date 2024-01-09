use std::error::Error;

mod models;
mod services;

pub mod trans {
    tonic::include_proto!("transaction");
}

use trans::GetRequest;

use crate::trans::{transactor_client::TransactorClient, AddRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = match TransactorClient::connect("http://127.0.0.1:50051").await {
        Ok(client) => client,
        Err(e) => {
            println!("Error connecting to transaction server: {}", e);
            return Ok(());
        }
    };

    println!("Connected to transaction server");

    let request = tonic::Request::new(GetRequest {
        id: "1".to_string(),
    });

    let new_transaction = tonic::Request::new(AddRequest {
        amount: 100.0,
        transaction_type: "debit".to_string(),
        timestamp: 123.to_string(),
        user: "josh".to_string(),
    });

    println!("Sending transaction");
    let res = match client.add(new_transaction).await {
        Ok(res) => res,
        Err(e) => {
            println!("Error sending add request to transaction server");
            println!("{}", e);
            return Ok(());
        }
    };
    println!("Transaction sent");

    let success = res.into_inner().success;
    println!("Transaction Response {}", success);

    let res = match client.get(request).await {
        Ok(res) => res,
        Err(e) => {
            println!("Error sending get request to transaction server");
            println!("{}", e);
            return Ok(());
        }
    };

    println!("Transaction Response {}", res.into_inner().id);

    Ok(())
}
