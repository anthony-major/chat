use clap::Parser;

use tokio::net::TcpListener;
use tokio::sync::broadcast::channel;
use tokio::signal;

use crate::message::Message;

pub async fn run() {
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}...", args.port);
    println!("Press ctrl+c to exit.");

    let (tx, _) = channel::<UserMessage>(16);

    loop {
        tokio::select! {
            future = listener.accept() => {
                let (stream, addr) = future?;
                tokio::spawn(handle_client(stream, addr, tx.clone(), tx.subscribe()));
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Exiting...");
    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}
