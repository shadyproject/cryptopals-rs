use crate::utils;

/// Challenge 1 is to take a string as an input and return the bas64 value, operating on the byte values
pub fn run() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let bytes = utils::hex_string_to_bytes(input);
    let b64 = base64::encode(bytes.unwrap());

    println!("Challenge 01");
    println!("Input: {}", input);
    println!("Base64: {}", b64);
    println!("Value is expected Base64 string: {}", b64 == expected);
    println!("----------")
}
