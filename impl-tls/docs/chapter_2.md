# いざ実装

echo server作ろう

そしたらやり取りができる！

tokio
ioをうまくやるための非同期ランタイムだから当然ネットワークのIOもやれるはず！

ってことでTCP Echoを作ってみよう

Server
https://docs.rs/tokio/latest/tokio/#examples

Client
https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#examples


exampleを参考にして、サーバーとクライアント、やりとりができるものを最小で作ろう

```
cargo run -p server

cargo run -p client