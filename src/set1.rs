use std::collections::HashMap;

use counter::Counter;

use crate::{decode_hex, encode_hex};

extern crate base64;

// TODO: should these functions only return vec<u8> and have helper functions for doing the string displays?

/// Challenge 1 is to take a string as an input and return the bas64 value, operating on the byte values
pub fn challenge1(input: &str) -> String {
    let bytes = decode_hex(input);
    base64::encode(bytes.unwrap())
}

/// Challenge 2 Write a function that takes two equal-length buffers and produces their XOR combination.
pub fn challenge2(input: &str, key: &str) -> String {
    let in_bytes = decode_hex(input).unwrap();
    let key_bytes = decode_hex(key).unwrap();
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
    let mut plaintext: Vec<u8> = Vec::new();
    let mut min_fq: Option<f64> = None;

    for k in 0u8..=255 {
        let text = single_byte_xor(input.as_bytes(), k);
        let fq = fitting_quotient(
            //TODO don't clone to fix the borrow checker
            &String::from_utf8(text.clone()).expect("Unable to convert plaintext bytes to string"),
        );

        match min_fq {
            Some(min) => {
                if fq < min {
                    key = k;
                    plaintext = text;
                    min_fq = Some(fq);
                }
            }
            None => {
                key = k;
                plaintext = text;
                min_fq = Some(fq);
            }
        }
    }

    (
        key,
        String::from_utf8(plaintext).expect("Unable to convert plaintext to string"),
    )
}

/// Fitting Quotient is the measure that suggests how well the two Letter Frequency Distributions match
/// For the second frequency distribution we use the standard english distibution
fn fitting_quotient(text: &str) -> f64 {
    let english = frequency_english();

    let counter = text.chars().collect::<Counter<_>>();
    let mut frequency_text: HashMap<char, f64> = HashMap::new();

    let text_len =
        u32::try_from(text.chars().count()).expect("Lenght of text too big to fit in u32");

    for c in english.keys() {
        match counter.get(&c) {
            Some(count) => {
                let uc = u32::try_from(*count).expect("Unable to convert count to u32");
                let fc = f64::try_from(uc).expect("Unable to convter u32 to f32");
                let f = fc * 100.0 / text_len as f64;
                frequency_text.insert(*c, f);
            }
            None => {
                let f = 0.0 / text_len as f64;
                frequency_text.insert(*c, f);
            }
        }
    }

    let freq_len =
        u32::try_from(frequency_text.len()).expect("Unable to convert frequency length to u32");
    frequency_text
        .values()
        .zip(english.values())
        .map(|t| f64::abs(t.0 - t.1))
        .sum::<f64>()
        / freq_len as f64
}

/// ETAOIN SHRDLU
fn frequency_english() -> HashMap<char, f64> {
    HashMap::from([
        ('a', 8.2389258),
        ('b', 1.5051398),
        ('c', 2.8065007),
        ('d', 4.2904556),
        ('e', 12.813865),
        ('f', 2.2476217),
        ('g', 2.0327458),
        ('h', 6.1476691),
        ('i', 6.1476691),
        ('j', 0.1543474),
        ('k', 0.7787989),
        ('l', 4.0604477),
        ('m', 2.4271893),
        ('n', 6.8084376),
        ('o', 7.5731132),
        ('p', 1.9459884),
        ('q', 0.0958366),
        ('r', 6.0397268),
        ('s', 6.3827211),
        ('t', 9.1357551),
        ('u', 2.7822893),
        ('v', 0.9866131),
        ('w', 2.3807842),
        ('x', 0.1513210),
        ('y', 1.9913847),
        ('z', 0.0746517),
    ])
}

pub fn single_byte_xor(plaintext: &[u8], key: u8) -> Vec<u8> {
    plaintext.iter().map(|b| b ^ key).collect()
}
