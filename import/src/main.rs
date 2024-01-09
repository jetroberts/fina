use clap::Parser;

mod cli;
mod server;
mod service;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    // start a web server
    #[arg(short, long, default_value = "false")]
    server: bool,
}

fn main() {
    let args = Args::parse();
    if args.server {
        server::start();
        return;
    }

    cli::start();
}
