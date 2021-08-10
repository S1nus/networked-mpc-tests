use paillier::EncodedCiphertext;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum Gm8sMessage {
    IntroMessage(usize),
    P0EncryptedPairs(Vec<encrypted_ab_pair>) ,
    P1EncryptedPairs(Vec<encrypted_ab_pair>) ,
    TP1P0(Vec<EncodedCiphertext<u64>>),
    TP2P0(Vec<EncodedCiphertext<u64>>),
    TP2P1(Vec<EncodedCiphertext<u64>>),
}

#[derive(Deserialize, Serialize)]
pub struct intro_message {
    pub player_num: usize,
}

// p0 sends array of these

#[derive(Deserialize, Serialize)]
pub struct encrypted_ab_pair {
    pub a: EncodedCiphertext<u64>,
    pub b: EncodedCiphertext<u64>,
}

pub type t_p1_p0 = EncodedCiphertext<u64>;


// p1 sends array of these

pub type t_p2_p0 = EncodedCiphertext<u64>;
pub type t_p2_p1 = EncodedCiphertext<u64>;
