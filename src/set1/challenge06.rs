use std::{fs::File, io::Read, path::Path};

/// Let's break repeating key xor
pub fn run() {
    println!("Challenge 06");
    let path = Path::new("./data/6.txt");
    let mut file = File::open(path).expect("Unable to open data file for challenge 06.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to read file data for challenge 06.");
    let ciphertext = base64::decode(buf).expect("Unable to decode base64 data.");

    println!("----------")
}

fn guess_keysize(ciphertext: &[u8]) -> u32 {
    let keysize = 0u32;

    // TODO: make this a function that takes a max keysize
    for ks in (2u32..=40) {
        ciphertext.chunks(ks as usize).take(2)
    }
}
