#![no_main]
#![no_std]

use risc0_zkvm_guest::env;
use hex_literal::hex;
use sha2::{Sha256, Digest};
use serde::__private::Vec;

risc0_zkvm_guest::entry!(main);

// Struct to be defined in future
//#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
//pub struct BlockHeader {
//    pub hash: u64, 
//}

pub fn main() {

    // Load the version of the block header
    let version: &[u8] = env::read();

    // Load the previous block's hash
    let previous_hash: &[u8] = env::read();

    // Load the merkle root
    let merkle_root: &[u8] = env::read();

    // Load the timestamp 
    let timestamp: u32 = env::read();

    // Load the bits/difficulty target
    let difficulty_target: &[u8] = env::read();

    // Load the nonce --- SECRET
    let nonce: &[u8] = env::read();

    // Load the serialized block header
    let header: &[u8] = env::read();
    // let header = hex!("0200000015a20d97f5a65e130e08f2b254f97f65b96173a7057aef0da203000000000000887e309c02ebdddbd0f3faff78f868d61b1c4cff2a25e5b3c9d90ff501818fa0e7965d508bdb051a40d8d8f7");

    // Compute the block's hash by double SHA256
    let mut hasher = Sha256::new();
    hasher.update(header);
    let hash1 = hasher.finalize();    

    let mut hasher2 = Sha256::new();
    hasher2.update(hash1);
    let result = hex::encode(hasher2.finalize());

    // Write the block header parameters into the public journal
    env::write(&version);
    env::write(&previous_hash);
    env::write(&merkle_root);
    env::write(&timestamp);
    env::write(&difficulty_target);

    // Commit the block's hash to the public journal
    // env::commit(&header);  // <~-------- Uncomment here to see truncated pre-hashed headerø
    env::commit(&result);
}
