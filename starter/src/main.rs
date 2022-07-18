use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use std::mem::transmute;
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use hex_literal::hex;
use sha2::{Sha256, Digest};

fn main() {

    // Blockchain Header Info
    let version = "00000002";
    let previous_hash = "00000000000003a20def7a05a77361b9657ff954b2f2080e135ea6f5970da215"; // hex 
    let merkle_root = "a08f8101f50fd9c9b3e5252aff4c1c1bd668f878fffaf3d0dbddeb029c307e88"; // hex
    let timestamp: u32 = 1348310759; // seconds
    let difficulty_target = "1a05db8b"; //
    let nonce = "f7d8d840"; // 

    // Decode Strings Into Hex and Reverse to Obtain Vectors in Little Endien
    let version_dec = hex::decode(version).expect("Decoding failed");
    let version_le: Vec<_> = version_dec.into_iter().rev().collect();

    let previous_hash_dec = hex::decode(previous_hash).expect("Decoding failed");
    let previous_hash_le: Vec<_> = previous_hash_dec.into_iter().rev().collect();

    let merkle_root_dec = hex::decode(merkle_root).expect("Decoding failed");
    let merkle_root_le: Vec<_> = merkle_root_dec.into_iter().rev().collect();

    let difficulty_target_dec = hex::decode(difficulty_target).expect("Decoding failed");
    let difficulty_target_le: Vec<_> = difficulty_target_dec.into_iter().rev().collect();

    let nonce_dec = hex::decode(nonce).expect("Decoding failed");
    let nonce_le: Vec<_> = nonce_dec.into_iter().rev().collect();

    // Convert Timestamp to Litte Endien Then Hex
    let le_timestamp: [u8; 4] = unsafe { transmute(timestamp.to_le()) } ;

    // Build the Block Header 
    let mut block_header = vec![];
    block_header.extend(version_le);
    block_header.extend(previous_hash_le);
    block_header.extend(merkle_root_le);
    block_header.extend(le_timestamp);
    block_header.extend(difficulty_target_le);
    block_header.extend(nonce_le);

    // START: Only for debugging purpose and to help evaluate my progress with this test
    let block_header_h = hex::encode(&block_header);
    println!("Serialized Block Header: {:?}", &block_header_h);
    let header = hex!("0200000015a20d97f5a65e130e08f2b254f97f65b96173a7057aef0da203000000000000887e309c02ebdddbd0f3faff78f868d61b1c4cff2a25e5b3c9d90ff501818fa0e7965d508bdb051a40d8d8f7");
    //println!("Serialized Block Header: {:?}", &header); 

    let mut hasher = Sha256::new();
    hasher.update(&block_header);
    let hash1 = hasher.finalize();
    let mut hasher2 = Sha256::new();
    hasher2.update(hash1);
    let hash2 = hex::encode(hasher2.finalize());
    println!("Expected Hashed Block Header: {}", hash2);
    // END 

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();

    // Next we send the block parameters to the guest
    prover.add_input(to_vec(&version).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&previous_hash).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&merkle_root).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&timestamp).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&difficulty_target).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&nonce).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&block_header_h).unwrap().as_slice()).unwrap(); // <!------------- Memory is truncated here

    // Run prover & generate receipt
    let receipt = prover.run().unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let block_hash: String = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

    // Print an assertion
    println!("I know the nonce for the block header: {}, and I can prove it!", block_hash);

    // Here is where one would send 'receipt' over the network...
    // ...

    // Verify receipt, panic if it's wrong
    receipt.verify(MULTIPLY_ID).unwrap();
}
