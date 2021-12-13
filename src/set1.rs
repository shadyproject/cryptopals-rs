use crate::{decode_hex, encode_hex};

extern crate base64;

// TODO: should these functions only return vec<u8> and have helper functions for doing the string displays?

/// Challenge 1 is to take a string as an input and return the bas64 value, operating on the byte values
pub fn challenge1(input: &str) -> String {
    let bytes = decode_hex(input);
    base64::encode(bytes.unwrap())
}

/// Challenge 2 is to take two equal size buffers and
pub fn challenge2(input: &str, key: &str) -> String {
    let in_bytes = decode_hex(input).unwrap();
    let key_bytes = decode_hex(key).unwrap();
    let mut result: Vec<u8> = Vec::with_capacity(in_bytes.len());

    for (idx, b) in in_bytes.into_iter().enumerate() {
        result.push(b ^ key_bytes[idx]);
    }

    encode_hex(&result)
}
