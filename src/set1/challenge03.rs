use std::collections::HashMap;

use crate::utils;
use crate::xor::XOR;

/// Challenge 3 is to find the 1 character key used to xor a string
pub fn run() {
    println!("Challenge 03");

    let ciphertext = utils::hex_string_to_bytes(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    )
    .expect("Unable to parse ciphertext hex string");

    let key = break_single_byte_xor(&ciphertext);
    let plaintext = ciphertext.xor(&[key]);

    println!(
        "Plaintext matches expected: {}",
        plaintext == b"Cooking MC's like a pound of bacon".as_ref()
    );
    println!("Got key: {}", key);
    println!("Got plaintext: {}", String::from_utf8(plaintext).unwrap());

    println!("----------");
}

fn break_single_byte_xor(input: &[u8]) -> u8 {
    (0u8..=255)
        .min_by_key(|&u| fitting_quotient(&input.xor(&[u])))
        .unwrap()
}

/// Fitting Quotient is the measure that suggests how well the two Letter Frequency Distributions match
pub fn fitting_quotient(input: &[u8]) -> u32 {
    if !input.is_ascii() {
        return std::u32::MAX;
    }

    if input.iter().any(|&c| is_control(c) && c != b'\n') {
        return std::u32::MAX;
    }

    let counts = count_characters(input);
    let length = input.len() as f32;

    ENGLISH_FREQUENCIES.iter().fold(0f32, |a, &(c, score)| {
        let expected_count = score / 100f32 * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        a + (expected_count - actual_count).powi(2)
    }) as u32
}

fn count_characters(chars: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();
    for &c in chars.iter() {
        if is_control(c) {
            continue;
        }
        let key = if is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t' {
            b' '
        } else {
            b'.'
        };

        let count = counts.entry(key).or_insert(0f32);
        *count += 1f32;
    }
    counts
}

fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}
// Source:
// Lee, E. Stewart. "Essays about Computer Security" (PDF). University of Cambridge Computer Laboratory. p. 181.
static ENGLISH_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];
