use rand::Rng;

use std::env;
use std::net::SocketAddrV4;
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
use futures_codec::{Bytes, LengthCodec, Framed, FramedWrite};

mod messages_types;
mod protocol;

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

async fn run_server(addr: SocketAddrV4, p1_data: &mut messages_types::Player1Data, p2_data: &mut messages_types::Player2Data) {
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to bind to {}", addr).as_str());

    println!("Listening on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Got a stream from {:?}!", addr);
        task::spawn(connection_loop(stream, p1_data, p2_data));
    }

}

async fn connection_loop(stream: TcpStream, p1_data: &mut messages_types::Player1Data, p2_data: &mut messages_types::Player2Data) {
    let mut framed = Framed::new(stream, LengthCodec);
    while let Some(message) = framed.try_next().await.expect("error with lengthcodec") {
        println!("{:?}", message);
    }
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

            let mut p1_data = messages_types::Player1Data {
                addr: None
            };

            task::block_on(run_server(
                SocketAddrV4::new(
                    "127.0.0.1"
                    .parse()
                    .unwrap(),
                4000),
                p1_data
            ));

        },
        1 => {
            let intro_message = messages_types::IntroMessage { player_num: 1 };
            println!("{}", serde_json::to_string(&intro_message).unwrap());
        },
        2 => {
        },
        _ => {
            panic!("Invalid player number supplied");
        }
    }

}
