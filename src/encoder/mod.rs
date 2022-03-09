use crate::{encode_table, wasm_bindgen};
use wasm_bindgen::prelude::*;

/// 入力された文字をゼロ幅文字を用いて2ビットエンコードします。
#[wasm_bindgen]
pub fn encode(str: &str) -> String {
    let bytes = str.as_bytes();
    let encode_table = crate::encode_table();
    bytes
        .iter()
        .map(|b| convert_to_zero_width(*b, &encode_table))
        .collect()
}

fn convert_to_zero_width(byte: u8, encode_table: &[char]) -> String {
    let zero_widths: String = (0..4)
        .map(|i| (byte >> 2 * i) & 0b00000011)
        .map(|bit| match bit {
            0 => encode_table[0],
            1 => encode_table[1],
            2 => encode_table[2],
            3 => encode_table[3],
            _ => panic!("Invalid binary digit: {}", bit),
        })
        .rev()
        .collect();
    zero_widths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "Hello World!";
        let expect = "\u{200d}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{200d}\u{200d}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{2062}\u{2062}\u{200c}\u{2060}\u{200c}\u{200c}\u{200d}\u{200d}\u{200d}\u{2062}\u{200d}\u{2060}\u{2062}\u{2062}\u{200d}\u{2062}\u{200c}\u{2060}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{200d}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}";
        assert_eq!(expect, encode(&input));
    }

    #[test]
    fn test_convert_to_zero_width() {
        let input: u8 = 0;

        let encode_table = encode_table();
        let expect: String = [
            encode_table[0],
            encode_table[0],
            encode_table[0],
            encode_table[0],
        ]
        .iter()
        .collect();
        assert_eq!(expect, convert_to_zero_width(input, &encode_table));

        let input: u8 = 4;
        let expect: String = [
            encode_table[0],
            encode_table[0],
            encode_table[1],
            encode_table[0],
        ]
        .iter()
        .collect();
        assert_eq!(expect, convert_to_zero_width(input, &encode_table));

        let input: u8 = 170;
        let expect: String = [
            encode_table[2],
            encode_table[2],
            encode_table[2],
            encode_table[2],
        ]
        .iter()
        .collect();
        assert_eq!(expect, convert_to_zero_width(input, &encode_table));

        let input: u8 = 255;
        let expect: String = [
            encode_table[3],
            encode_table[3],
            encode_table[3],
            encode_table[3],
        ]
        .iter()
        .collect();
        assert_eq!(expect, convert_to_zero_width(input, &encode_table));
    }
}
