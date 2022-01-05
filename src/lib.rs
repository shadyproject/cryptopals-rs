use counter::Counter;
use std::{collections::HashMap, fmt::Write, num::ParseIntError};
pub mod set1;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn single_byte_xor(text: &[u8], key: u8) -> Vec<u8> {
    text.iter().map(|b| b ^ key).collect()
}

/// Fitting Quotient is the measure that suggests how well the two Letter Frequency Distributions match
/// For the second frequency distribution we use the standard english distibution
pub fn fitting_quotient(text: &str) -> f64 {
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
pub fn frequency_english() -> HashMap<char, f64> {
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
