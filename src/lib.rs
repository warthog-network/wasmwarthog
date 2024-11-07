mod utils;
use getrandom::getrandom;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use k256::ecdsa::{ SigningKey, VerifyingKey};



#[wasm_bindgen]
pub fn generate_private_key() -> String {
    let mut key = [0u8; 32];
    getrandom(&mut key).expect("Failed to generate random key");
    hex::encode(key)
}

#[wasm_bindgen]
pub fn generate_pub_key(pk: String) -> String {
    let signing_key = SigningKey::from_slice(&hex::decode(pk).unwrap()).unwrap();
    hex::encode(VerifyingKey::from(signing_key).to_sec1_bytes())
}

#[wasm_bindgen]
pub fn generate_address(public_key : String) -> String {
    let sha = sha2::Sha256::digest(hex::decode(public_key).unwrap());
    let ripe_part = Ripemd160::digest(&sha);
    let hasher_checksum = sha2::Sha256::digest(&ripe_part);
    let checksum = &hasher_checksum[..4];
    let addr = [&ripe_part[..], checksum].concat();
    return hex::encode(addr);
}

#[wasm_bindgen]
pub fn fetch_tx_data() {
    todo!()
}

pub fn make_tx(pin_height : u32, pin_hash : String, nonce_id: u32, fee_e8 : u64, amount_e8: u64, to : String, pk: String) -> String{
    let mut to_sign :Vec<u8> = Vec::new();
    to_sign.extend(hex::decode(pin_hash).unwrap()); // decode hex string to bytes
    to_sign.extend(&pin_height.to_be_bytes());
    to_sign.extend(&nonce_id.to_be_bytes());
    // Fill with 3 bytes of 0
    to_sign.extend(&[0, 0, 0]);
    to_sign.extend(&fee_e8.to_be_bytes());
    // address that we delete 4 bytes of checksum
    let to_addr_bytes = hex::decode(to).unwrap();
    to_sign.extend(&to_addr_bytes[0..20]);
    to_sign.extend(&amount_e8.to_be_bytes());

    let signing_key = SigningKey::from_slice(&hex::decode(pk).unwrap()).unwrap();

    let result = signing_key.sign_digest_recoverable(Sha256::new_with_prefix(to_sign)).unwrap();

    let address = hex::encode(result.0.to_bytes());
    
    format!("{}{}", address, hex::encode(&[result.1.to_byte()]))
}