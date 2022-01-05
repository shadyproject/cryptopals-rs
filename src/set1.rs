use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{decode_hex, encode_hex, fitting_quotient, single_byte_xor};

extern crate base64;

// TODO: should these functions only return vec<u8> and have helper functions for doing the string displays?

/// Challenge 1 is to take a string as an input and return the bas64 value, operating on the byte values
pub fn challenge1(input: &str) -> String {
    let bytes = decode_hex(input);
    base64::encode(bytes.unwrap())
}

/// Challenge 2 Write a function that takes two equal-length buffers and produces their XOR combination.
pub fn challenge2(input: &str, key: &str) -> String {
    let in_bytes = decode_hex(input).expect("Unable to parse input hex string into bytes.");
    let key_bytes = decode_hex(key).expect("Unable to parse key hex string into bytes.");
    let mut result: Vec<u8> = Vec::with_capacity(in_bytes.len());

    for (idx, b) in in_bytes.into_iter().enumerate() {
        result.push(b ^ key_bytes[idx]);
    }

    encode_hex(&result)
}

/// Challenge 3 is to find the 1 character key used to xor a string
/// Returns a tuple with key and plaintext
pub fn challenge3(input: &str) -> (u8, String) {
    let mut key: u8 = 0;
    let mut plaintext: String = String::new();
    let mut min_fq: Option<f64> = None;

    let in_bytes = decode_hex(input).expect("Unable to parse input hex string into bytes.");
    // let result = best_cleartext_for_all_keys(&&in_bytes);

    for k in 0u8..=255 {
        println!("Testing key {}", k);
        let pt = single_byte_xor(&in_bytes, k);
        let t = String::from_utf8(pt);
        match t {
            Ok(text) => {
                let fq = fitting_quotient(&text);
                match min_fq {
                    Some(min) => {
                        if fq < min {
                            println!(
                                "New best fitting quotient: {}.  Previous: {}",
                                fq,
                                min_fq.unwrap()
                            );
                            key = k;
                            plaintext = text;
                            min_fq = Some(fq);
                            println!("Possible key found: {}", k);
                            println!("Possible plaintext: {}", plaintext);
                        }
                    }
                    None => {
                        key = k;
                        plaintext = text;
                        min_fq = Some(fq);
                    }
                }
            }
            Err(error) => {
                println!("Error while testing key: {}", error);
                continue;
            }
        }
    }

    (key, plaintext)
    //(result.0, result.2)
}

/// Challenge 4 is to detect a single character xor in a file with 60 entries
pub fn challenge4(input_file: File) -> String {
    let hexes = BufReader::new(input_file)
        .lines()
        .map(|l| {
            let line = l.expect("Unable to parse line from file.");
            decode_hex(&line).expect("Unable to parse bytes from hex string.")
        })
        .collect::<Vec<Vec<u8>>>();
    let mut min_fq: Option<f64> = None;
    let mut output: String = String::new();

    for hex in hexes {
        let result = best_cleartext_for_all_keys(&hex);

        match min_fq {
            None => (min_fq = Some(result.1)),
            Some(min) => {
                if result.1 < min {
                    min_fq = Some(min);
                    output = result.2;
                }
            }
        }
    }

    output
}

/// Get the best plaintext and fitting quotient for all single byte keys
/// Returns a tuple with key, fitting quotient, and plaintext
pub fn best_cleartext_for_all_keys(ciphertext: &[u8]) -> (u8, f64, String) {
    let mut plaintext: String = String::new();
    let mut min_fq: Option<f64> = None;
    let mut k: u8 = 0;

    for key in 0..=255 {
        let clear = single_byte_xor(ciphertext, key);
        k = key;
        match String::from_utf8(clear) {
            Err(_) => (continue),
            Ok(text) => {
                let fq = fitting_quotient(&text);
                match min_fq {
                    Some(min) => {
                        if fq < min {
                            plaintext = text;
                            min_fq = Some(fq);
                        }
                    }
                    None => {
                        plaintext = text;
                        min_fq = Some(fq);
                    }
                }
            }
        }
    }

    (k, min_fq.unwrap_or(0.0), plaintext)
}
