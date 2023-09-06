use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    // Read data from stream.
    let mut reader = BufReader::new(stream);
    let mut buffer = [0; 1024];
    let length = reader.read(&mut buffer).await?;

    println!("{}", String::from_utf8_lossy(&buffer[..length]));

    //let mut buffer = Vec::new();
    //stream.read_buf(&mut buffer).await?;

    //dbg!(String::from_utf8(buffer)?);

    Ok(())
}
// とりあえず echo サーバ作ってる感じですよね？
// イエス
// かしこまりでし！

// 一旦僕のイメージで書いてみて良いですか 1分ください
// いけーーーーー

// これうごかんすかね？
// いった！！！

// まいど手直しあいざいます...
// なるへそ〜、使ったことなかったです
// ぴーす
// わーーーい
// わーーい
