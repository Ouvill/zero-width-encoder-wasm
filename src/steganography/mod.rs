use crate::{decode, encode, encode_table};
use js_sys;
use wasm_bindgen::prelude::*;

/// ゼロ幅文字を入力されたテキストに埋め込みます｡
#[wasm_bindgen]
pub fn embed(text: &str, hidden: &str) -> String {
    let steganography = encode(hidden);
    let char_count = text.chars().count();
    let center = char_count / 2;

    let (first, last) = text.split_at(center);

    // match (front, back) {
    //     (Some(front), Some(back)) => {
    //         let front = front.to_string();
    //         let back = back.to_string();
    //         let text = front + &steganography + &back;
    //         return text;
    //     }
    //     (_, _) => {
    //         print!("error");
    //         return text.len().to_string();
    //     }
    // }

    let embed = format!("{}{}{}", first, steganography, last);
    embed
}

fn regex(input: &str, char_list: &[char]) -> Option<js_sys::Array> {
    let pattern = format!("[{}]+", char_list.iter().collect::<String>());
    let regex = js_sys::RegExp::new(&pattern, "g");
    let result = regex.exec(&input);
    result
}

/// ゼロ幅文字が埋め込まれたテキストからデータを検出します。
#[wasm_bindgen]
pub fn detect(input: &str) -> js_sys::Array {
    let encode_table = encode_table();
    let matches = regex(input, &encode_table);
    let result = match matches {
        Some(m) => m,
        None => return js_sys::Array::new(),
    };

    let mut decoded = vec![];
    for item in result.iter() {
        let match_str = item.as_string().unwrap();
        let decoded_str = decode(&match_str);
        if let Ok(decoded_str) = decoded_str {
            decoded.push(decoded_str);
        }
    }

    convert_to_js_array(&decoded)
}

pub fn convert_to_js_array(input: &Vec<String>) -> js_sys::Array {
    let array = js_sys::Array::new();
    for item in input {
        let item = js_sys::JsString::from(item.clone());
        array.push(&item);
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_embed() {
        let input = "foo bar";
        let hidden = "Hello World!";
        let expected = "foo\u{200c}\u{200b}\u{200d}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{2060}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{2060}\u{200c}\u{2060}\u{200b}\u{200d}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200b}\u{200d}\u{200b}\u{200c} bar".to_string();
        assert_eq!(embed(input, hidden), expected)
    }

    #[test]
    fn test_embed_non_string() {
        let input = "";
        let hidden = "Hello World!";
        let expected = "\u{200c}\u{200b}\u{200d}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{2060}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{2060}\u{200c}\u{2060}\u{200b}\u{200d}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200b}\u{200d}\u{200b}\u{200c}".to_string();
        assert_eq!(embed(input, hidden), expected)
    }

    #[test]
    fn test_embed_to_1_char() {
        let input = "a";
        let hidden = "Hello World!";
        let expected = "\u{200c}\u{200b}\u{200d}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{2060}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{2060}\u{200c}\u{2060}\u{200b}\u{200d}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200b}\u{200d}\u{200b}\u{200c}a".to_string();
        assert_eq!(embed(input, hidden), expected)
    }

    #[test]
    fn test_embed_to_3_char() {
        let input = "abc";
        let hidden = "Hello World!";
        let expected = "a\u{200c}\u{200b}\u{200d}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{2060}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{2060}\u{200c}\u{2060}\u{200b}\u{200d}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200b}\u{200d}\u{200b}\u{200c}bc".to_string();
        assert_eq!(embed(input, hidden), expected)
    }

    #[wasm_bindgen_test]
    fn test_detect() {
        let input = "hou are\u{200c}\u{200b}\u{200d}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{2060}\u{2060}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{2060}\u{200c}\u{2060}\u{200b}\u{200d}\u{200c}\u{200d}\u{2060}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200b}\u{200d}\u{200b}\u{200c} you";

        let hello = js_sys::JsString::from("Hello World!");
        let expected = js_sys::Array::new();
        expected.push(&hello);

        assert_eq!(detect(input), expected)
    }
}
