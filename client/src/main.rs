use tokio::io::{self, BufReader, BufWriter, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use clap::Parser;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.address, args.port);

    println!("Connecting to {}...", addr);
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected.");

    let (read_stream, write_stream) = stream.split();

    let mut buf_read_stream = BufReader::new(read_stream);
    let mut buf_write_stream = BufWriter::new(write_stream);
    let mut stream_input = String::new();

    let mut buf_reader = BufReader::new(io::stdin());
    let mut user_input = String::new();

    loop {
        tokio::select! {
            _ = buf_reader.read_line(&mut user_input) => {
                user_input = user_input.replace("\\n", "\n");

                if user_input.trim_end() == "quit" {
                    break;
                }

                println!("Sending message:\n{}", user_input);
                buf_write_stream.write(user_input.as_bytes()).await?;
                buf_write_stream.flush().await?;
                println!("Sent.");

                user_input.clear();
            }
            _ = buf_read_stream.read_line(&mut stream_input) => {
                println!("Read message:\n{}", stream_input);

                stream_input.clear();
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}
