use std::net::SocketAddr;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use clap::Parser;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}...", args.port);
    println!("Press ctrl+c to exit.");

    let (tx, _) = channel::<String>(16);

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

async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<String>,
    mut rx: Receiver<String>,
) -> io::Result<()> {
    println!("{} connected.", addr);

    let (read_stream, mut write_stream) = io::split(stream);

    let mut lines = BufReader::new(read_stream).lines();

    loop {
        tokio::select! {
            line = lines.next_line() => {
                match line {
                    Ok(line) => {
                        match line {
                            Some(line) => {
                                println!("[{}:{}] {}", addr.ip(), addr.port(), line);
                                tx.send(line).expect("Failed to send through tx.");
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
            message = rx.recv() => {
                let mut message = message.expect("Failed to read from rx.");
                message.push('\n');

                println!("Broadcasting to {}...", addr);
                write_stream.write(message.as_bytes()).await.expect("Failed to write to stream.");
                write_stream.flush().await.unwrap();
                println!("Broadcasted to {}.", addr);
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
