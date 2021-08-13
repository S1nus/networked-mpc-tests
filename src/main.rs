use rand::Rng;

use std::env;
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

mod messages_types;
mod protocol;

fn p0() {
    const NUM_TRIPLES: usize = 300;
    let p = 700;
    let mut rng = rand::thread_rng();

    println!("generating my keypair for Paillier's cryptosystem...");
    let (ek, dk) = Paillier::keypair().keys();
    println!("keypair generated.");

    println!("generating my {} random triples...", NUM_TRIPLES);
    let pairs: Vec<protocol::ABPair> = (0..NUM_TRIPLES).map(|_| {
        protocol::ABPair {
            a: rng.gen_range(0..p),
            b: rng.gen_range(0..p),
        }
    }).collect();
    println!("Done generating my {} random triples!", NUM_TRIPLES); 
    println!("Encrypting my 300 random triples with Paillier's Encryption...");

    let encrypted_pairs: Vec<messages_types::EncryptedABPair> = pairs
        .iter()
        .map(|pair| {
            messages_types::EncryptedABPair {
                a: Paillier::encrypt(&ek, pair.a),
                b: Paillier::encrypt(&ek, pair.b),
            }
        })
        .collect();
    println!("Done encrypting {} random triples!", NUM_TRIPLES);
    println!("Size of encrypted triples: {}", size_of::<[paillier::EncodedCiphertext<u32>; NUM_TRIPLES]>());

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let player_number : u8 = args[1]
        .parse()
        .expect("unable to parse supplied player number arg");

    match player_number {
        0 => {
            p0();
        },
        1 => {
        },
        2 => {
        },
        _ => {
            panic!("Invalid player number supplied");
        }
    }

}
