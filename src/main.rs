fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("Input: {}", input);
    println!("Base64: {}", cryptopals_rs::set1::hex_to_base64(input));
}
