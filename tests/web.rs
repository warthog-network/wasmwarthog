//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::{debug_assert, println};

use wasm_bindgen_test::{__rt::{__wbgtest_console_debug, console_log}, console_log, *};
use wasmwarthog::{generate_address, generate_private_key, generate_pub_key, make_tx};


wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn generate_private_key_test() {
    assert!(hex::decode(generate_private_key()).unwrap().len() == 32);
}


#[wasm_bindgen_test]
fn generate_pub_and_private_key_and_address() {
    let private_key = generate_private_key();
    let pubkey = generate_pub_key(private_key.clone());
    let address = generate_address(pubkey.clone());
    console_log!("PK :");
    console_log!("{}", private_key);
    console_log!("Pubkey :");
    console_log!("{}", pubkey);
    console_log!("Adresse");
    console_log!("{}", address);
}

#[wasm_bindgen_test]
fn create_sig() {
    let private_key = "966a71a98bb5d13e9116c0dffa3f1a7877e45c6f563897b96cfd5c59bf0803e0".to_string();
    let pin_hash = "863b1c21c2361b657d825811f69cf229a3b1d362a9870803bd06f0ea02085689".to_string();
    let pinHeight = 2150592;
    let sig = make_tx(pinHeight, pin_hash, 0, 9992, 100000000, "0000000000000000000000000000000000000000de47c9b2".to_string(), private_key);
    console_log!("{}", sig.clone());
    console_log!("{}", hex::decode(sig.clone()).unwrap().len());
    assert!(hex::decode(sig.clone()).unwrap().len() == 65)
    
}