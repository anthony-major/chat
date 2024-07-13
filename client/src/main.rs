use clap::Parser;
use client::Client;

use crate::client_ui::App;

mod client;
mod client_ui;
mod message;
mod protocol;

#[tokio::main]
async fn main() -> eframe::Result {
    let args = Args::parse();
    let addr = format!("{}:{}", args.address, args.port);

    let client = Client::connect(addr, args.username).await.unwrap();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat - egui",
        options,
        Box::new(|_| Ok(Box::new(App::new(client)))),
    )
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value_t = 9000)]
    port: u16,

    #[arg(short, long, default_value = "User")]
    username: String,
}
