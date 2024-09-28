use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::signal;
use tokio::sync::broadcast::channel;
use tokio::sync::Mutex;

use crate::client_handler::handle_client;
use crate::message::Message;

pub async fn run<A: ToSocketAddrs>(address: A, message_bound: usize) -> io::Result<()> {
    let listener = TcpListener::bind(address).await?;
    println!("Listening on port {}...", listener.local_addr()?.port());
    println!("Press ctrl+c to exit.");

    let (tx, _) = channel::<Message>(message_bound);
    let users: Arc<Mutex<HashMap<String, SocketAddr>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        tokio::select! {
            connection = listener.accept() => {
                let (stream, address) = connection?;
                tokio::spawn(handle_client(stream, address, tx.clone(), tx.subscribe(), users.clone()));
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Exiting...");
    Ok(())
}
