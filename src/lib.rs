extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const ZERO : char  = '\u{200B}';
const ONE : char  = '\u{200C}';

/// 入力された文字をゼロ幅文字'u{200B}'と'u{200C}'を用いてバイナリエンコードします。
/// 'u{200B}'を0、'u{200C}'を1としてエンコードします。
#[wasm_bindgen]
pub fn encode(str: &str) -> String {
    let bytes = str.as_bytes();
    bytes.iter().map(|b| {
        let encoded = convert_to_zero_width(*b);
        encoded
    }).collect()
}

fn convert_to_zero_width(byte : u8) -> String {
    let zero_widths: String = (0..8)
        .map(|i|  ( byte >> i) & 0b00000001 )
        .map(|bit| match bit {
            0 => ZERO,
            1 => ONE,
            _ => panic!("Invalid binary digit: {}", bit),
        })
        .rev()
        .collect();
    zero_widths
}

/// ゼロ幅文字('u{200B}','u{200C}')のバイナリエンコードされた文字列をUTF8でデコードします｡
#[wasm_bindgen]
pub fn decode(string: &str) -> String {
    let count = string.chars().count();
    let bytes_count = count / 8;
    // 1バイトずつ処理
    (0..bytes_count).map(|i| {
        let start = i * 8;
        let binary = string.chars().skip(start).take(8).collect::<String>();
        let decoded = convert_from_zero_width(&binary);
        decoded
    }).collect::<Vec<u8>>().iter().map(|&b| b as char).collect::<String>()
}

fn convert_from_zero_width(string: &str) -> u8 {
    string.chars().map(|c| {
        match c {
            ZERO => 0,
            ONE => 1,
            _ => panic!("Invalid zero width character: {}", c),
        }
    }).fold(0, |acc, x| acc << 1 | x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "Hello World!";
        let expect = "\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}";
        assert_eq!(expect, encode(&input));
    }

    #[test]
    fn test_decode() {
        let input = "\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}";
        let expect = "Hello World!";
        assert_eq!(expect, decode(&input));
    }

    #[test]
    fn test_convert_from_zero_width() {
        let input: String =  [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].iter().collect();
        let expect = 0;
        assert_eq!(expect, convert_from_zero_width(&input));

        let input: String = [ONE, ZERO, ONE, ZERO, ONE, ZERO, ONE, ZERO].iter().collect();
        let expect = 170;
        assert_eq!(expect, convert_from_zero_width(&input));


        let input: String = [ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE].iter().collect();
        let expect = 255;
        assert_eq!(expect, convert_from_zero_width(&input));
    }

    #[test]
    fn test_convert_to_zero_width() {
        let input: u8 = 0;
        let expect: String = [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].iter().collect();
        assert_eq!(expect , convert_to_zero_width(input));

        let input: u8 = 4;
        let expect: String = [ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO].iter().collect();
        assert_eq!(expect, convert_to_zero_width(input));

        let input: u8 = 170;
        let expect: String = [ONE, ZERO, ONE, ZERO, ONE, ZERO, ONE, ZERO].iter().collect();
        assert_eq!(expect , convert_to_zero_width(input));

        let input: u8 = 255;
        let expect: String = [ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE].iter().collect();
        assert_eq!(expect , convert_to_zero_width(input));
    }
}
