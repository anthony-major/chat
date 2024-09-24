use std::net::SocketAddr;

use tokio::io::{self, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::message::Message;
use crate::protocol::{read_message, write_message};

pub async fn handle_client(
    stream: TcpStream,
    address: SocketAddr,
    tx: Sender<Message>,
    mut rx: Receiver<Message>,
) -> io::Result<()> {
    println!("{} connected.", address);

    let (read_stream, write_stream) = io::split(stream);

    let mut buf_read_stream = BufReader::new(read_stream);
    let mut buf_write_stream = BufWriter::new(write_stream);

    println!("{} waiting for username...", address);
    let username = match read_message(&mut buf_read_stream).await {
        Err(e) => {
            println!("{} {}.", address, e);
            return Ok(());
        }
        Ok(message) => message.username().clone(),
    };
    println!("{} received username {}.", address, username);
    let _ = tx.send(Message::new(
        String::from("server"),
        format!("{} connected.", username),
    ));

    loop {
        tokio::select! {
            message = read_message(&mut buf_read_stream) => {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{} {} {}.", address, username, e);
                        break;
                    }
                };

                if message.username() != &username {
                    println!("{} gave incorrect username. Expected '{}' but received '{}'.", address, username, message.username());
                    break;
                }

                println!("[{} {}] {:?}", address, username, message);
                let _ = tx.send(message).map_err(|e| println!("{} {} {}", address, username, e));
            }
            message = rx.recv() => {
                let message = match message {
                    Err(RecvError::Closed) => {
                        break;
                    }
                    Err(RecvError::Lagged(skipped_message_count)) => {
                        println!("{} {} lagging by {} messages.", address, username, skipped_message_count);
                        continue;
                    }
                    Ok(message) => message
                };

                println!("Broadcasting to {}...", address);
                write_message(&mut buf_write_stream, message).await?;
                println!("Broadcasted to {}.", address);
            }
        }
    }

    let disconnected_message = Message::new(
        String::from("server"),
        format!("{} disconnected.", username),
    );
    if let Err(e) = tx.send(disconnected_message) {
        println!("{} {} {}", address, username, e);
    }

    Ok(())
}
