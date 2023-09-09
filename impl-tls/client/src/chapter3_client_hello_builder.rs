use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut payload = Vec::<u8>::new();
    loop {}
}

struct ClientHello {
    client_version: ClientVersion,
    client_random: ClientRandom,
    session_id: SessionId,
    cipher_suiters: CipherSuites,
    compression_methods: CompressionMethods,
    extentions: Vec<Box<dyn Extension>>,
}

enum ClientVersion {
    Tls13,
}

impl Encodable for ClientVersion {
    fn encode(&self) -> Vec<u8> {
        match self {
            ClientVersion::Tls13 => vec![0x03, 0x04],
        }
    }
}

struct ClientRandom {
    random_bytes: [u8; 32],
}

impl Encodable for ClientRandom {
    fn encode(&self) -> Vec<u8> {
        self.random_bytes.to_vec()
    }
}

impl Decodable for ClientRandom {
    fn decode<const N: usize>(bytes: &[u8; N]) -> Self {
        Self {
            random_bytes: bytes.iter(),
        }
    }
}
// iteratorで取得するとかできそう
// 32個までの要素を取得するとかはダメそう？

struct SessionId(Vec<u8>);

impl Encodable for SessionId {
    fn encode(&self) -> Vec<u8> {
        let length = self.0.len();
        let mut bytes = Vec::with_capacity(length + 1);
        bytes.push(length as u8);
        bytes.extend_from_slice(&self.0);
        bytes
    }
}

enum CipherSuites {}

enum CompressionMethods {}

trait Extension {}

trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

trait Decodable {
    fn decode<const N: usize>(bytes: &[u8; N]) -> Self;
}

// trait Packet とかしたら楽しそうだなって思ったので
// 書いた瞬間に後からやった方がいいかな〜？とか思ったりしたので・・・
// Encodableの方が何やるかわかりやすいかも
