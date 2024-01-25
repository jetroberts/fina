use clap::Parser;

mod cli;
mod database;
mod service;
mod web;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    // start a web server
    #[arg(short = 's', long, default_value = "false")]
    enable_server: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.enable_server {
        let server = web::Server::new();
        match server.start().await {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        };
        return;
    }

    let cli = cli::Cli::new();
    cli.start();
}
