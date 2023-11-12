use std::{error::Error, time::Instant};

mod models;
mod services;

pub mod trans {
    tonic::include_proto!("transaction");
}

use trans::GetRequest;

use crate::trans::transactor_client::TransactorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TransactorClient::connect("http://[127.0.0.1]:50051").await?;

    loop {
        let request = tonic::Request::new(GetRequest {
            id: "1".to_string(),
        });

        let now = Instant::now();
        let _res = client.get(request).await?;
        println!("TIME={:?}", now.elapsed());
    }
}
