

const ZERO : char  = '\u{200B}';
const ONE : char  = '\u{200C}';

pub fn encode(str: &str) -> String {
    let bytes = str.as_bytes();
    bytes.iter().map(|&b| {
        let binary = bytes_to_binary(b);
        let encoded = convert_to_zero_width(&binary);
        encoded
    }).collect()
}

fn bytes_to_binary (bytes: u8) -> String {
    let binary = format!("{:b}", bytes);
    let len = binary.chars().count();
    // pad with zeros
    let zeros = (0..(8-len)).map(|_| '0')
        .collect::<String>();
    format!("{}{}", zeros, binary)
}

fn convert_to_zero_width(binary: &str) -> String {
    binary.chars().map(|c| {
        match c {
            '0' => ZERO,
            '1' => ONE,
            _ => panic!("Invalid binary digit: {}", c),
        }
    }).collect::<String>()
}


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
        let input = "00000000";
        let expect: String = [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].iter().collect();
        assert_eq!(expect , convert_to_zero_width(&input));

        let input = "11111111";
        let expect: String = [ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE].iter().collect();
        assert_eq!(expect , convert_to_zero_width(&input));

        let input = "10101010";
        let expect: String = [ONE, ZERO, ONE, ZERO, ONE, ZERO, ONE, ZERO].iter().collect();
        assert_eq!(expect , convert_to_zero_width(&input));
    }

    #[test]
    fn test_bytes_to_binary() {
        assert_eq!("00000000",bytes_to_binary(0));
        assert_eq!("00000001",bytes_to_binary(1));
        assert_eq!("00000010",bytes_to_binary(2));
        assert_eq!("00100000",bytes_to_binary(32));
        assert_eq!("11111111",bytes_to_binary(255));
    }
}
