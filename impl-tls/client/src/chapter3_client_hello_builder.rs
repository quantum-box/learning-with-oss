use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use byteorder::{NetworkEndian, WriteBytesExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut payload = Vec::<u8>::new();
    loop {}
}

struct ClientHello {
    pub client_version: ClientVersion,
    pub client_random: ClientRandom,
    pub session_id: SessionId,
    pub cipher_suiters: CipherSuites,
    pub compression_methods: CompressionMethods,
    pub extentions: Vec<Box<dyn Extension>>,
}

impl Encodable for ClientHello {
    fn encode<W: std::io::Write>(&self, write: &mut W) -> usize {
        let mut sum = 0;
        sum += self.client_version.encode(write);
        sum += self.client_random.encode(write);
        sum += self.session_id.encode(write);
        sum += self.cipher_suiters.encode(write);
        sum += self.compression_methods.encode(write);

        for ext in self.extentions.iter() {
            sum += ext.encode(write);
        }

        sum
    }
}

enum ClientVersion {
    Tls13, // 0x0304
}

macro_rules! write_prim {
    ($writer: ident, $write_func: ident, $val: expr) => {{
        fn get_type_size_helper<T>(_: T) -> usize {
            std::mem::size_of::<T>()
        }

        let val = $val;
        $writer.$write_func::<NetworkEndian>(val).unwrap();
        get_type_size_helper(val)
    }};
    {$writer: ident, write_u8, $val: expr} => {{
        let val = $val;
        $writer.$write_func(val).unwrap();
        get_type_size_helper(val)
    }}
}

impl Encodable for ClientVersion {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        match *self {
            Self::Tls13 => write_prim!(writer, write_u16, 0x0304),
        }
    }
}

struct ClientRandom {
    random_bytes: [u8; 32],
}

impl Encodable for ClientRandom {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        writer.write_all(&self.random_bytes);
        self.random_bytes.len()
    }
}

impl Decodable for ClientRandom {
    fn decode(bytes: &[u8; 32]) -> Self {
        Self {
            random_bytes: bytes.clone(),
        }
    }
}
// iteratorで取得するとかできそう
// 32個までの要素を取得するとかはダメそう？

struct SessionId(Vec<u8>);

impl Encodable for SessionId {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        writer.write_all(&self.0).unwrap();
        self.0.len()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum SuiteEntry {
    TLS_AES_256_GCM_SHA384,
    TLS_CHACHA20_POLY1305_SHA256,
    TLS_AES_128_GCM_SHA256,
    TLS_EMPTY_RENEGOTIATION_INFO_SCSV,
}

impl Into<u16> for &SuiteEntry {
    fn into(self) -> u16 {
        match self {
            SuiteEntry::TLS_AES_256_GCM_SHA384 => 0x1302,
            SuiteEntry::TLS_CHACHA20_POLY1305_SHA256 => 0x1303,
            SuiteEntry::TLS_AES_128_GCM_SHA256 => 0x1301,
            SuiteEntry::TLS_EMPTY_RENEGOTIATION_INFO_SCSV => 0x00ff,
        }
    }
}

impl Encodable for SuiteEntry {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        write_prim!(writer, write_u16, self.into())
    }
}

struct CipherSuites {
    suites: Vec<SuiteEntry>,
}

impl Encodable for CipherSuites {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        let mut sum = 0;
        sum += write_prim!(writer, write_u16, self.suites.len() as u16);

        for ent in self.suites.iter() {
            sum += ent.encode(writer);
        }

        sum
    }
}

struct CompressionMethods;

impl Encodable for CompressionMethods {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize {
        writer.write_all(&[0x01, 0x00]);
        2
    }
}

trait Extension {}

trait Encodable {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> usize;
}

trait Decodable {
    fn decode(bytes: &[u8]) -> Self;
}

// trait Packet とかしたら楽しそうだなって思ったので
// 書いた瞬間に後からやった方がいいかな〜？とか思ったりしたので・・・
// Encodableの方が何やるかわかりやすいかも

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_xargs_client_hello() {
        let client_hello = ClientHello {
            client_version: ClientVersion::Tls13,
            client_random: ClientRandom {
                random_bytes: [
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                    0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
                    0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
                ],
            },
            session_id: SessionId(vec![
                0x20, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec,
                0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa,
                0xfb, 0xfc, 0xfd, 0xfe, 0xff,
            ]),
            cipher_suiters: CipherSuites {
                suites: vec![SuiteEntry::TLS_AES_256_GCM_SHA384],
            },
            compression_methods: CompressionMethods,
            extentions: vec![],
        };
    }
}
