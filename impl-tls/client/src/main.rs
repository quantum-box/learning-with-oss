use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    // Read data from stream.
    let mut buffer = Vec::new();
    stream.read_buf(&mut buffer).await?;

    dbg!(String::from_utf8(buffer)?);

    Ok(())
}
