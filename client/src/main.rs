use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::signal;

use serde::{Deserialize, Serialize};

use clap::Parser;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.address, args.port);

    let mut handles: Vec<JoinHandle<Result<(), io::Error>>> = Vec::new();

    for i in 0..2 {
        let handle = tokio::spawn(run(addr.clone(), i.to_string()));
        handles.push(handle);
    }

    for handle in handles {
        handle.await?.unwrap();
    }

    Ok(())
}

async fn run(addr: String, username: String) -> io::Result<()> {
    println!("Connecting to {}...", addr);
    let stream = TcpStream::connect(addr).await?;
    println!("Connected.");

    let (read_stream, write_stream) = io::split(stream);

    let mut buf_read_stream = BufReader::new(read_stream);
    let mut buf_write_stream = BufWriter::new(write_stream);
    let mut stream_input = Vec::<u8>::new();

    let handle1 = tokio::spawn(async move {
        for i in 0..16 {
            let message = UserMessage {
                username: username.clone(),
                content: format!("{} {}", username, i),
            };
            let mut message = serde_json::to_string(&message).unwrap();
            message.push('\0');

            println!("{} sending message {}...", username, i);
            buf_write_stream.write(message.as_bytes()).await.unwrap();
            buf_write_stream.flush().await.unwrap();
            println!("{} Sent {}.\n", username, i);
        }
    });

    let handle2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    break;
                }
                _ = buf_read_stream.read_until(b'\0', &mut stream_input) => {
                    println!("Read message:\n{}\n", String::from_utf8(stream_input).unwrap());
                    stream_input = Vec::<u8>::new();
                }
            }
        }
    });

    handle1.await?;
    handle2.await?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct UserMessage {
    username: String,
    content: String,
}

// Old loop for getting and sending input from stdin
// loop {
//     tokio::select! {
//         // _ = buf_reader.read_line(&mut user_input) => {
//         //     if user_input.trim_end() == "quit" {
//         //         break;
//         //     }

//         //     user_input = user_input.replace("\\n", "\n");
//         //     user_input.pop();
//         //     user_input.push('\0');

//         //     println!("Sending message:\n{}", user_input);
//         //     buf_write_stream.write(user_input.as_bytes()).await?;
//         //     buf_write_stream.flush().await?;
//         //     println!("Sent.");

//         //     user_input.clear();
//         // }
//         _ = buf_read_stream.read_until(b'\0', &mut stream_input) => {
//             println!("Read message:\n{}", String::from_utf8(stream_input).unwrap());

//             stream_input = Vec::<u8>::new();
//         }
//     }
// }
