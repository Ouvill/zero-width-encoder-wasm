use crate::{decode, encode_table};
use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn embed() -> String {
    // TODO
    String::from("")
}

#[wasm_bindgen]
pub fn detect(input: &str) -> js_sys::Array {
    let encode_table = encode_table();
    let pattern = format!("[{}]+", encode_table.iter().collect::<String>());
    let regex = js_sys::RegExp::new(&pattern, "g");
    let matches = regex.exec(input);
    let result = match matches {
        Some(m) => m,
        None => return js_sys::Array::new(),
    };
    let decoded = js_sys::Array::new();
    for item in result.iter() {
        let match_str = item.as_string().unwrap();
        let decoded_str = decode(&match_str);
        if let Ok(decoded_str) = decoded_str {
            let decoded_str = js_sys::JsString::from(decoded_str);
            decoded.push(&decoded_str);
        }
    }

    decoded
}

#[derive(PartialEq, Debug)]
struct Foo {
    bar: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_detect() {
        let input = "hou are\u{200d}\u{200c}\u{2060}\u{200c}\u{200d}\u{2060}\u{200d}\u{200d}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{2062}\u{2062}\u{200c}\u{2060}\u{200c}\u{200c}\u{200d}\u{200d}\u{200d}\u{2062}\u{200d}\u{2060}\u{2062}\u{2062}\u{200d}\u{2062}\u{200c}\u{2060}\u{200d}\u{2060}\u{2062}\u{200c}\u{200d}\u{2060}\u{200d}\u{200c}\u{200c}\u{2060}\u{200c}\u{200d} you";

        let hello = js_sys::JsString::from("Hello World!");
        let expected = js_sys::Array::new();
        expected.push(&hello);

        assert_eq!(detect(input), expected)
    }
}
