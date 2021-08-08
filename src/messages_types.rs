use paillier::EncodedCipherText;
use serde_json::de::{Deserialize, Serialize};

#[Derive(Deserialize, Serialize)]
pub struct intro_message {
    pub player_num: usize,
}

// p0 sends array of these

#[Derive(Deserialize, Serialize)]
pub struct p0_encrypted_pair {
    pub x: EncodedCipherText<u64>,
    pub y: EncodedCipherText<u64>,
}

#[Derive(Deserialize, Serialize)]
pub type t_p1_p0: EncodedCipherText<u64>;

#[Derive(Deserialize, Serialize)]
pub type p1_encrypted_pair {
    pub x: EncodedCipherText<u64>,
    pub y: EncodedCipherText<u64>,
}

#[Derive(Deserialize)]
pub type t_p2_p0: EncodedCipherText<u64>;
pub type t_p2_p1: EncodedCipherText<u64>;
