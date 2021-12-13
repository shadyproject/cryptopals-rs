use crate::decode_hex;

extern crate base64;

pub fn hex_to_base64(input: &str) -> String {
    let bytes = decode_hex(input);
    base64::encode(bytes.unwrap())
}
