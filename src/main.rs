use rand::Rng;

use std::env;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};

use async_std::prelude::*;
use async_std::io::prelude::BufReadExt;

use async_std::{task,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    stream::{Stream, StreamExt},
};

use serde_json::de::Deserializer;

use paillier::*;

use std::mem::size_of;

use futures::stream::TryStreamExt;
use futures::SinkExt;
use futures_codec::{Bytes, BytesMut, LengthCodec, Framed, FramedWrite, Decoder, Encoder};
use std::io::{Error, ErrorKind};

mod messages_types;
mod protocol;

pub struct MyStringCodec(LengthCodec);

impl Encoder for MyStringCodec {
    type Item = String;
    type Error = Error;

    fn encode(&mut self, src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = Bytes::from(src);
        self.0.encode(bytes, dst)
    }
}

impl Decoder for MyStringCodec {
    type Item = String;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.0.decode(src)? {
            Some(bytes) => {
                match String::from_utf8(bytes.to_vec()) {
                    Ok(string) => Ok(Some(string)),
                    Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
                }
            },
            None => Ok(None),
        }
    }
}


fn generate_triples(num: usize, p: u64) -> Vec<protocol::ABPair> {
    let mut rng = rand::thread_rng();

    let pairs: Vec<protocol::ABPair> = (0..num).map(|_| {
        protocol::ABPair {
            a: rng.gen_range(0..p),
            b: rng.gen_range(0..p),
        }
    }).collect();
    println!("Done generating my {} random triples!", num); 

    pairs

}

fn encrypt_triples(triples: Vec<protocol::ABPair> , ek: &paillier::EncryptionKey, num_triples: usize) -> Vec<messages_types::EncryptedABPair> {
    println!("Encrypting my 300 random triples with Paillier's Encryption...");

    let encrypted_pairs: Vec<messages_types::EncryptedABPair> = triples
        .iter()
        .map(|pair| {
            messages_types::EncryptedABPair {
                a: Paillier::encrypt(ek, pair.a),
                b: Paillier::encrypt(ek, pair.b),
            }
        })
        .collect();
    println!("Done encrypting {} random triples!", num_triples);

    encrypted_pairs
}

async fn run_server(addr: SocketAddrV4, p1_data: Arc<Mutex<messages_types::PlayerData>>, p2_data: Arc<Mutex<messages_types::PlayerData>>) {
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to bind to {}", addr).as_str());

    println!("Listening on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Got a stream from {:?}!", addr);
        task::spawn(connection_loop(stream, p1_data.clone(), p2_data.clone()));
    }

}

async fn connection_loop(stream: TcpStream, p1_data: Arc<Mutex<messages_types::PlayerData>>, p2_data: Arc<Mutex<messages_types::PlayerData>>) {
    let mut framed = Framed::new(stream, MyStringCodec(LengthCodec));

    if let Some(first_message) = framed.try_next().await.expect("error parsing first message with lengthcodec") {
        let identify : messages_types::IntroMessage = serde_json::from_str(&first_message).expect("failed to parse intro message");
        match identify.player_num {
            0 => {
                println!("that's not right. I'm player 0!");
            },
            1 => {
                println!("connected to player 1");
            },
            2 => {
                println!("Connected to player 2");
            },
            _ => {
                panic!("invalid message");
            }
        }
    }

    while let Some(message) = framed.try_next().await.expect("error with lengthcodec") {
        println!("{:?}", message);
    }
}

async fn connect_to_player(addr: SocketAddrV4) -> TcpStream {
    TcpStream::connect(addr)
        .await
        .expect(&format!("failed to connect to player: {}", addr))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let player_number : u8 = args[1]
        .parse()
        .expect("unable to parse supplied player number arg");

    match player_number {
        0 => {
            const NUM_TRIPLES: usize = 300;
            let p = 700;

            println!("generating my keypair for Paillier's cryptosystem...");
            let (ek, dk) = Paillier::keypair().keys();
            println!("keypair generated.");

            println!("generating my {} random triples...", NUM_TRIPLES);
            let triples = generate_triples(NUM_TRIPLES, p);
            let encrypted_triples = encrypt_triples(triples, &ek, NUM_TRIPLES);

            let p1_data = Arc::new(Mutex::new(
                    messages_types::PlayerData {
                        addr: None
                    }
            ));

            let p2_data = Arc::new(Mutex::new(
                    messages_types::PlayerData {
                        addr: None
                    }
            ));


            task::block_on(run_server(
                SocketAddrV4::new(
                    "127.0.0.1"
                    .parse()
                    .unwrap(),
                4000),
                p1_data,
                p2_data
            ));

        },
        1 => {
            let intro_message = messages_types::IntroMessage { player_num: 1 };
            let intro_message_string = serde_json::to_string(&intro_message).unwrap();
            let mut p0_stream = task::block_on(connect_to_player(
                    SocketAddrV4::new("127.0.0.1"
                    .parse()
                    .unwrap()
                    , 4000)
            ));
            let mut framed_write = FramedWrite::new(p0_stream, MyStringCodec(LengthCodec));
            task::block_on(
                framed_write.send(intro_message_string)
            )
            .expect("failed to send");
        },
        2 => {
            let intro_message = messages_types::IntroMessage { player_num: 1 };
        },
        _ => {
            panic!("Invalid player number supplied");
        }
    }

}
