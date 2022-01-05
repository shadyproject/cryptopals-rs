use crate::{utils::hex_string_from_bytes, xor::XOR};

pub fn run() {
    println!("Challenge 05");
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c632\
    4202d623d63343c2a26226324272765272a282b2f2\
    0430a652e2c652a3124333a653e2b2027630c692b2\
    0283165286326302e27282f";

    let ciphertext = input.xor(key);
    println!(
        "Encrypted value equals expected value: {}",
        hex_string_from_bytes(&ciphertext) == expected
    );
    println!("Ciphertext: {}", hex_string_from_bytes(&ciphertext));

    println!("----------")
}

/// repeating key xor
/// In repeating-key XOR, you'll sequentially apply each byte of the key;
/// the first byte of plaintext will be XOR'd against first byte of the key,
/// the next byte of plaintext the next byte of the key,
/// when the end of the key length is reached, return to the first byte of the key
#[allow(dead_code)]
fn xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());

    for chunk in input.chunks(key.len()) {
        for (idx, plaintext_byte) in chunk.iter().enumerate() {
            result.push(plaintext_byte ^ key[idx])
        }
    }

    result
}
