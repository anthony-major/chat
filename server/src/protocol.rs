use std::net::SocketAddr;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use crate::message::Message;

pub async fn read_message(
    buf_reader: &mut BufReader<ReadHalf<TcpStream>>,
    address: &SocketAddr,
) -> Result<Message, io::Error> {
    let mut raw_message = Vec::<u8>::new();

    match buf_reader.read_until(b'\0', &mut raw_message).await {
        Err(e) => {
            return Err(e);
        }
        Ok(bytes_read) if bytes_read == 0 => {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("{} disconnected.", address),
            ));
        }
        Ok(_) => {
            let message = match String::from_utf8(raw_message) {
                Ok(mut message) => {
                    message.pop();
                    message
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
                }
            };

            let message: Message = match serde_json::from_str(&message) {
                Ok(message) => message,
                Err(e) => {
                    return Err(e.into());
                }
            };

            return Ok(message);
        }
    }
}

pub async fn write_message(
    buf_writer: &mut BufWriter<WriteHalf<TcpStream>>,
    message: Message,
) -> io::Result<()> {
    let mut message = serde_json::to_string(&message)?;
    message.push('\0');

    let mut message = message.as_bytes();

    buf_writer.write_all_buf(&mut message).await?;
    buf_writer.flush().await?;

    Ok(())
}
