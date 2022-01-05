use std::{fs::File, io::BufRead, io::BufReader, path::Path};

use crate::{utils::hex_string_to_bytes, xor::XOR};

use super::challenge03::fitting_quotient;

/// Challenge 04 is basically challenge 03 but for a whole bunch of potential inputs.
pub fn run() {
    println!("Challenge 04");
    let path = Path::new("data/4.txt");
    let file = File::open(path).expect("Unable to open data file for challenge 04.");
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|l| {
            hex_string_to_bytes(l.expect("Unable to read line from file.").trim())
                .expect("Unable to parse hex string.")
        })
        .collect::<Vec<Vec<u8>>>();

    let mut min_fq = u32::MAX;
    let mut result: Vec<u8> = Vec::new();

    for line in lines {
        for k in 0u8..=255 {
            let plaintext = line.xor(&[k]);
            let fq = fitting_quotient(&plaintext);
            if fq < min_fq {
                min_fq = fq;
                result = plaintext;
            }
        }
    }

    println!(
        "Result equals expected: {}",
        b"Now that the party is jumping\n".as_ref() == result
    );
    println!("Result: {}", String::from_utf8(result).unwrap());

    println!("----------")
}
