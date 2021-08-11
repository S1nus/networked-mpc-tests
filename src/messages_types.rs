use paillier::EncodedCiphertext;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum Gm8sMessage {
    IntroMessage(usize),
    P0EncryptedPairs(Vec<EncryptedABPair>) ,
    P1EncryptedPairs(Vec<EncryptedABPair>) ,
    TP1P0(Vec<EncodedCiphertext<u64>>),
    TP2P0(Vec<EncodedCiphertext<u64>>),
    TP2P1(Vec<EncodedCiphertext<u64>>),
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedABPair {
    pub a: EncodedCiphertext<u64>,
    pub b: EncodedCiphertext<u64>,
}
