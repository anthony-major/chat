use std::net::SocketAddr;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;

use clap::Parser;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}...", args.port);
    println!("Press ctrl+c to exit.");

    loop {
        tokio::select! {
            future = listener.accept() => {
                let (stream, addr) = future?;
                tokio::spawn(handle_client(stream, addr));
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Exiting...");
    Ok(())
}

async fn handle_client(stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
    println!("{} connected.", addr);

    let (read_stream, _write_stream) = io::split(stream);

    let mut lines = BufReader::new(read_stream).lines();

    loop {
        match lines.next_line().await {
            Ok(line) => {
                match line {
                    Some(line) => {
                        println!("[{}:{}] {}", addr.ip(), addr.port(), line);
                    }
                    _ => {
                        break;
                    }
                }
            }
            Err(e) => {
                println!("{} forcefully disconnected.", addr);
                return Err(e);
            }
        }
    }

    println!("{} disconnected.", addr);
    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}
