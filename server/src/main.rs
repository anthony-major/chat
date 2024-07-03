use tokio::io;

use clap::Parser;

use crate::server::run;

mod client_handler;
mod message;
mod protocol;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let address = format!("127.0.0.1:{}", args.port);

    run(address, 16).await?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}
