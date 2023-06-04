/// A latin-1 tokenizer
use std::cmp::Ordering;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ISO_8859_1;


/// Converts a string slice input into a vector of u8 integers
pub fn encode(input: &str, vector_size: usize) -> Vec<u8> {
    let encoded = ISO_8859_1
    .encode(input, EncoderTrap::Strict)
    .expect("Tokenizer Encoder Error when converting characters to bytes");
    
    match encoded.len().cmp(&vector_size) {
        Ordering::Less => {
            let padding_size = vector_size - encoded.len();
            let zeros: Vec<u8>  = vec![0; padding_size];
            [encoded, zeros].concat()
        },
        Ordering::Greater => encoded[..vector_size].to_vec(),
        Ordering::Equal => encoded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let output = vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
        let padded_zeros: Vec<u8> = vec![0; 2];

        assert_eq!(encode("Hello, world!", 10), output[..10]);
        assert_eq!(encode("Hello, world!", 13), output);
        assert_eq!(encode("Hello, world!", 15), [output, padded_zeros].concat());
    }
}
