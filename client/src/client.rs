use std::error::Error;
use std::thread;

use tokio::io::{self, BufReader, BufWriter, ReadHalf, WriteHalf};
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

        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(async move {
                let stream = TcpStream::connect(server_address)
                    .await
                    .expect("Failed to connect.");
                let (read_half, write_half) = io::split(stream);

                Self::run(
                    BufReader::new(read_half),
                    BufWriter::new(write_half),
                    sending_channel.0,
                    receiving_channel.1,
                )
                .await
                .unwrap();
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
