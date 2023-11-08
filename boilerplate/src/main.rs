use std::error::Error;

mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
