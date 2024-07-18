use std::error::Error;
use std::thread;

use tokio::io::{self, AsyncWriteExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::message::Message;
use crate::protocol::{read_message, write_message};

pub struct Client {
    external_sender: Sender<Message>,
    external_receiver: Receiver<Message>,
    username: String,
}

impl Client {
    pub fn connect(server_address: String, username: String) -> Self {
        let sending_channel = mpsc::channel::<Message>(16);
        let receiving_channel = mpsc::channel::<Message>(16);

        let username_clone = username.clone();
        thread::spawn(|| {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(async move {
                let stream = TcpStream::connect(server_address)
                    .await
                    .expect("Failed to connect.");
                let (read_half, write_half) = io::split(stream);

                let reader = BufReader::new(read_half);
                let mut writer = BufWriter::new(write_half);

                // First we need to send a message to the server to tell it our username and fully connect
                let username_message = Message::new(username_clone, String::new());
                write_message(&mut writer, username_message)
                    .await
                    .expect("Failed to send username.");

                match Self::run(reader, writer, sending_channel.0, receiving_channel.1).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                };
            });
        });

        Self {
            external_sender: receiving_channel.0,
            external_receiver: sending_channel.1,
            username: username,
        }
    }

    async fn run(
        mut reader: BufReader<ReadHalf<TcpStream>>,
        mut writer: BufWriter<WriteHalf<TcpStream>>,
        sender: Sender<Message>,
        mut receiver: Receiver<Message>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            tokio::select! {
                message = read_message(&mut reader) => {
                    let message = message?;
                    sender.send(message).await?;
                }
                message = receiver.recv() => {
                    match message {
                        Some(message) => {
                            write_message(&mut writer, message).await?;
                        }
                        None => {
                            return Err("Receiver closed.".into());
                        }
                    }
                }
            }
        }
    }

    pub fn read(&mut self) -> &mut Receiver<Message> {
        &mut self.external_receiver
    }

    pub fn send(&self) -> &Sender<Message> {
        &self.external_sender
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}
