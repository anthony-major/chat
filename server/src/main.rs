mod message;
mod server;

use std::net::SocketAddr;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, ReadHalf};
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

async fn read_message(buf_reader: &mut BufReader<ReadHalf<TcpStream>>, addr: &SocketAddr) -> Result<UserMessage, io::Error> {
    let mut raw_message = Vec::<u8>::new();

    match buf_reader.read_until(b'\0', &mut raw_message).await {
        Err(e) => {
            return Err(e);
        }
        Ok(bytes_read) if bytes_read == 0 => {
            return Err(io::Error::new(io::ErrorKind::ConnectionAborted, format!("{} disconnected.", addr)));
        }
        Ok(_) => {
            let message = match String::from_utf8(raw_message) {
                Ok(mut message) => {
                    message.pop();
                    message
                },
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
                }
            };

            let message: UserMessage = match serde_json::from_str(&message) {
                Ok(message) => message,
                Err(e) => {
                    return Err(e.into());
                }
            };

            return Ok(message);
        }
    }
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

    println!("{} waiting for username...", addr);
    let username = match read_message(&mut buf_read_stream, &addr).await {
        Err(e) => {
            println!("{} {}", addr, e);
            return Ok(());
        }
        Ok(message) => {
            message.username
        }
    };
    println!("{} received username {}.", addr, username);

    loop {
        tokio::select! {
            message = read_message(&mut buf_read_stream, &addr) => {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{} {} {}", addr, username, e);
                        break;
                    }
                };

                println!("[{} {}] {:?}", addr, username, message);
                tx.send(message).expect("Failed to send through tx.");
            }
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
