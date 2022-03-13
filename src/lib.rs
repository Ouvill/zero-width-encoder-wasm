extern crate wasm_bindgen;

mod decoder;
mod encoder;
mod steganography;

pub use decoder::decode;
pub use encoder::encode;
pub use steganography::detect;
pub use steganography::embed;

enum ZWC {
    ZeroWidthSpace,
    ZeroWidthNonJoiner,
    ZeroWidthJoiner,
    WordJoiner,
    InvisibleTimes,
    // 文字方向の指示を元に戻す
    PopDirectionalFormatting,
    FunctionApplication,
    InvisibleSeparator,

    // ファイルのエンコーディングを示すBOMとしても利用されているので非推奨
    ZERO_WIDTH_NON_BREAK,
    // iOS, IEで表示される
    InvisiblePlus,
}

impl ZWC {
    fn value(&self) -> char {
        match *self {
            ZWC::ZeroWidthSpace => '\u{200B}',
            ZWC::ZeroWidthNonJoiner => '\u{200C}',
            ZWC::ZeroWidthJoiner => '\u{200D}',
            ZWC::PopDirectionalFormatting => '\u{202C}',
            ZWC::WordJoiner => '\u{2060}',
            ZWC::FunctionApplication => '\u{2061}',
            ZWC::InvisibleTimes => '\u{2062}',
            ZWC::InvisibleSeparator => '\u{2063}',
            ZWC::InvisiblePlus => '\u{2064}',
            ZWC::ZERO_WIDTH_NON_BREAK => '\u{FEFE}',
        }
    }
}

fn encode_table() -> [char; 4] {
    [
        ZWC::ZeroWidthSpace.value(),
        ZWC::ZeroWidthNonJoiner.value(),
        ZWC::ZeroWidthJoiner.value(),
        // ZWC::PopDirectionalFormatting.value(),
        ZWC::WordJoiner.value(),
        // ZWC::InvisibleTimes.value(),
        // ZWC::InvisibleSeparator.value(),
        // ZWC::ZERO_WIDTH_NON_BREAK.value(),
    ]
}

#[cfg(test)]
mod tests {}
