use tokio::io::{self, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use crate::message::Message;
use crate::protocol::{read_message, write_message};

pub struct Client {
    reader: BufReader<ReadHalf<TcpStream>>,
    writer: BufWriter<WriteHalf<TcpStream>>,
    username: String,
    messages: Vec<Message>,
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
        })
    }

    pub async fn read_message(&mut self) -> io::Result<()> {
        let message = read_message(&mut self.reader).await?;
        self.messages.push(message);
        Ok(())
    }

    pub async fn send_message(&mut self, message: Message) -> io::Result<()> {
        write_message(&mut self.writer, message).await
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}
