use clap::Parser;

use iced::{Application, Settings, Size};

use crate::client::Client;

mod client;
mod message;
mod protocol;

fn main() -> iced::Result {
    // let args = Args::parse();
    // let addr = format!("{}:{}", args.address, args.port);

    let mut settings = Settings::default();
    settings.window.size = Size::new(750.0, 500.0);

    Client::run(settings)
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}
