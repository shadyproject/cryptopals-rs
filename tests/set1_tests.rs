use cryptopals_rs;

#[test]
fn test_challenge1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(expected, cryptopals_rs::set1::challenge1(input));
}

#[test]
fn test_challenge2() {
    let input = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    assert_eq!(expected, cryptopals_rs::set1::challenge2(input, key))
}
