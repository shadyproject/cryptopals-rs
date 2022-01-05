use std::fs::File;

fn main() {
    do_challenge_1();
    do_challenge_2();
    do_challenge_3();
    // do_challenge_4();
}

fn print_challenge_header(challenger_number: u32) {
    println!("Challenge {}", challenger_number);
}

fn print_challenge_footer() {
    println!("----------")
}

fn do_challenge_1() {
    print_challenge_header(1);
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("Input: {}", input);
    println!("Base64: {}", cryptopals_rs::set1::challenge1(input));
    print_challenge_footer();
}

fn do_challenge_2() {
    print_challenge_header(2);
    let input = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";

    println!("Input: {}", input);
    println!("XOR: {}", cryptopals_rs::set1::challenge2(input, key));
    print_challenge_footer();
}

fn do_challenge_3() {
    print_challenge_header(3);
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("Input: {}", input);

    let result = cryptopals_rs::set1::challenge3(input);
    println!(
        "Recovered Key: {}\nRecovered Plaintext: {}",
        result.0, result.1
    );
    print_challenge_footer();
}

fn do_challenge_4() {
    print_challenge_header(4);

    let path = "./data/4.txt";
    match File::open(path) {
        Err(e) => (println!("Error opening input file: {}", e)),
        Ok(file) => {
            let result = cryptopals_rs::set1::challenge4(file);
            println!("Most probable xor-ed string is: {}", result);
        }
    }
    print_challenge_footer();
}
