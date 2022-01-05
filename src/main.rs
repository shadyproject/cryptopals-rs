fn main() {
    do_challenge_1();
    do_challenge_2();
    do_challenge_3();
}

fn do_challenge_1() {
    println!("Challenge 1");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("Input: {}", input);
    println!("Base64: {}", cryptopals_rs::set1::challenge1(input));
    println!("----------")
}

fn do_challenge_2() {
    println!("Challenge 2");
    let input = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";

    println!("Input: {}", input);
    println!("XOR: {}", cryptopals_rs::set1::challenge2(input, key));
    println!("----------")
}

fn do_challenge_3() {
    println!("Challenge 3");
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("Input: {}", input);

    let result = cryptopals_rs::set1::challenge3(input);
    println!(
        "Recovered Key: {}\nRecovered Plaintext: {}",
        result.0, result.1
    );
    println!("----------")
}
