use std::net::SocketAddr;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use clap::Parser;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> io::Result<()> {
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

async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<UserMessage>,
    mut rx: Receiver<UserMessage>,
) -> io::Result<()> {
    println!("{} connected.", addr);

    let (read_stream, mut write_stream) = io::split(stream);

    let mut buf_read_stream = BufReader::new(read_stream);
    let mut raw_message = Vec::<u8>::new();

    loop {
        tokio::select! {
            bytes_read = buf_read_stream.read_until(b'\0', &mut raw_message) => match bytes_read {
                Ok(bytes_read) if bytes_read > 0 => {
                    let message = match String::from_utf8(raw_message) {
                        Ok(mut message) => {
                            message.pop();
                            message
                        },
                        Err(e) => {
                            println!("{}:{} {}", addr.ip(), addr.port(), e);
                            raw_message = Vec::<u8>::new();
                            continue;
                        }
                    };

                    let message: UserMessage = match serde_json::from_str(&message) {
                        Ok(message) => message,
                        Err(e) => {
                            println!("{} Error parsing json:\n{}", addr, e);
                            raw_message = Vec::<u8>::new();
                            continue;
                        }
                    };

                    println!("[{}:{}]\n{:?}", addr.ip(), addr.port(), message);
                    tx.send(message).expect("Failed to send through tx.");

                    raw_message = Vec::<u8>::new();
                }
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    println!("{} forcefully disconnected.", addr);
                    return Err(e);
                }
            },
            message = rx.recv() => {
                let message = message.expect("Failed to read from rx.");
                let mut message = serde_json::to_string(&message).unwrap();
                message.push('\0');

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserMessage {
    username: String,
    content: String,
}
