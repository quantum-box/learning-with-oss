# いざ実装

echo server作ろう

そしたらやり取りができる！

tokio
ioをうまくやるための非同期ランタイムだから当然ネットワークのIOもやれるはず！

ってことでTCP Echoを作ってみよう

Server
https://docs.rs/tokio/1.32.0/tokio/index.html#examples

Client
https://docs.rs/tokio/1.32.0/tokio/net/struct.TcpStream.html#examples


exampleを参考にして、サーバーとクライアント、やりとりができるものを最小で作ろう

```shell
cargo run -p server
cargo run -p client
```

[コミット](https://github.com/quantum-box/learning-with-oss/commit/22d49ec482805b937474e973ec74a8b5e623250c)

dbg!でログを仕込み以下を実行する
```shell
cargo run -p server
cargo run -p client
```

以下のようなログが表示される
```shell
learning-with-oss/impl-tlscargo run -p server
   Compiling server v0.1.0 (/Users/megumish/github/learning-with-oss/impl-tls/server)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/server`
[server/src/main.rs:26] n = 12
[server/src/main.rs:27] &buf[0..n] = [
    104,
    101,
    108,
    108,
    111,
    32,
    119,
    111,
    114,
    108,
    100,
    33,
]
```

