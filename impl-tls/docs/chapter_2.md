# いざ実装

echo server 作ろう

そしたらやり取りができる！

tokio
io をうまくやるための非同期ランタイムだから当然ネットワークの IO もやれるはず！

ってことで TCP Echo を作ってみよう

Server
https://docs.rs/tokio/1.32.0/tokio/index.html#examples

Client
https://docs.rs/tokio/1.32.0/tokio/net/struct.TcpStream.html#examples

example を参考にして、サーバーとクライアント、やりとりができるものを最小で作ろう

```shell
cargo run -p server --bin chapter2_echo
cargo run -p client --bin chapter2_echo
```

[コミット](https://github.com/quantum-box/learning-with-oss/commit/22d49ec482805b937474e973ec74a8b5e623250c)

println!で標準出力にログを仕込み以下を実行する

```shell
cargo run -p server --bin chapter2_echo
cargo run -p client --bin chapter2_echo
```

以下のようなログが表示される

```shell
$ cargo run -p server --bin chapter2_echo
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/chapter2_echo`
received message: hello world!
```

```shell
$ cargo run -p client --bin chapter2_echo
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/chapter2_echo`
received message: hello world!
```
