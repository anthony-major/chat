use std::io::{self, Write};
use std::net::TcpStream;

use clap::Parser;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.address, args.port);

    println!("Connecting to {}...", addr);
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected.");

    let mut input = String::new();

    loop {
        print!(">");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        input = input.replace("\\n", "\n");

        if input.trim_end() == "quit" {
            break;
        }

        println!("Sending message:\n{}", input);
        stream.write(input.as_bytes())?;
        println!("Sent.");

        input.clear();
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
