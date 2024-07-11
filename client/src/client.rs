use tokio::io::{self, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use crate::message::Message;
use crate::protocol::{read_message, write_message};

pub struct Client {
    reader: BufReader<ReadHalf<TcpStream>>,
    writer: BufWriter<WriteHalf<TcpStream>>,
    username: String,
    messages: Vec<Message>,
    running: bool,
}

impl Client {
    pub async fn connect(server_address: String, username: String) -> io::Result<Self> {
        let stream = TcpStream::connect(server_address).await?;
        let (read_half, write_half) = io::split(stream);

        Ok(Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
            username: username,
            messages: Vec::new(),
            running: true,
        })
    }

    pub async fn run_read_loop(&mut self) -> io::Result<()> {
        while self.running {
            let message = read_message(&mut self.reader).await?;
            self.messages.push(message);
        }

        Ok(())
    }

    pub async fn send_message(&mut self, message: Message) -> io::Result<()> {
        write_message(&mut self.writer, message).await
    }

    pub fn stop_read_loop(&mut self) {
        self.running = false;
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}
