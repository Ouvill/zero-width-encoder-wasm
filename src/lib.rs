extern crate wasm_bindgen;

mod decoder;
mod encoder;

pub use decoder::decode;
pub use encoder::encode;

enum ZWC {
    ZeroWidthNonJoiner,
    ZeroWidthJoiner,
    WordJoiner,
    InvisibleTimes,
    InvisibleSeparator,
    // ファイルのエンコーディングを示すBOMとしても利用されているので非推奨
    // ZERO_WIDTH_NON_BREAK,

    // iOS, IEで表示される
    // InvisiblePlus
}

impl ZWC {
    fn value(&self) -> char {
        match *self {
            ZWC::ZeroWidthNonJoiner => '\u{200C}',
            ZWC::ZeroWidthJoiner => '\u{200D}',
            ZWC::WordJoiner => '\u{2060}',
            ZWC::InvisibleTimes => '\u{2062}',
            ZWC::InvisibleSeparator => '\u{2063}',
            // ZWC::ZERO_WIDTH_NON_BREAK => '\u{FEFE}',
            // ZWC::InvisiblePlus => '\u{2064}',
        }
    }
}

fn encode_table() -> [char; 4] {
    [
        ZWC::ZeroWidthNonJoiner.value(),
        ZWC::ZeroWidthJoiner.value(),
        ZWC::WordJoiner.value(),
        ZWC::InvisibleTimes.value(),
    ]
}

#[cfg(test)]
mod tests {}
