use core::panic;

use crate::utils::{decode_hex, encode_hex};

/// Challenge 2 Write a function that takes two equal-length buffers and produces their XOR combination.
pub fn run() {
    println!("Challenge 02");
    let expected = "746865206b696420646f6e277420706c6179";

    let clear = decode_hex("1c0111001f010100061a024b53535009181c")
        .expect("Unable to decode hex input string.");
    let key = decode_hex("686974207468652062756c6c277320657965")
        .expect("Unable to decode hey key string.");
    let cipher = xor(&clear, &key);

    println!(
        "Value is expected encrypted bytes: {}",
        encode_hex(&cipher) == expected
    );
    println!("----------")
}

fn xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    // this probably isnt true, i think for xor we just wont encrypt the rest of the bytes
    if input.len() != key.len() {
        panic!("input and key length must be the same for xor");
    }
    input
        .iter()
        .zip(key.iter())
        .map(|pair| pair.0 ^ pair.1)
        .collect()
}
