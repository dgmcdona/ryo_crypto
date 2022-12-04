pub mod convert {
    use core::fmt;
    use itertools::Itertools;
    use std::error::Error;
    use std::fmt::{Debug, Display};
    use std::u8;

    static B64CHARS: &'static [char] =
        &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
        'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
        '4', '5', '6', '7', '8', '9', '0', '+', '/'];

    fn bytes_from_hex(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if input.len() % 2 != 0 {
            return Err(Box::new(FormatError::HexStrError("hex string must be of even length")));
        }

        match input
            .chars()
            .chunks(2)
            .into_iter()
            .map(|x| u8::from_str_radix(x.collect::<String>().as_str(), 16))
            .collect() {
                Ok(bytes) => return Ok(bytes),
                Err(e) => return Err(Box::new(e)),
            }
    }

    pub fn hex_str_to_b64(input: &str) -> Result<String, Box<dyn Error>> {
        let bytes = bytes_from_hex(input)?;
        Ok(bytes_to_b64(bytes))
    }

    fn chars_to_b64(input: &[u8]) -> String {
        let mut chrs = String::new();

        chrs.push(B64CHARS[(input[0] >> 2) as usize]); // first

        if input.len() >= 2 {
            let v = ((input[0] & 0x3) << 4)| ((input[1] & 0xf0) >> 4);
            chrs.push(B64CHARS[v as usize]); // second
        } else {
            chrs.push(B64CHARS[((input[0] & 0x3) << 4) as usize]); // second
            chrs.push('=');
            chrs.push('=');
            return chrs;
        }

        if input.len() >= 3 {
            let v = ((input[1] & 0xf) << 2) | (((input[2] & 0xc0) >> 6) & 0xf);
            chrs.push(B64CHARS[v as usize]); // third
        } else {
            chrs.push(B64CHARS[((input[1] & 0xf) << 2) as usize]); // third
            chrs.push('=');
            return chrs;
        }

        chrs.push(B64CHARS[(input[2] & 0x3f) as usize]);

        chrs
    }

    pub fn bytes_to_b64(input: Vec<u8>) -> String {
        input
            .chunks(3)
            .into_iter()
            .map(|i| chars_to_b64(i))
            .collect()
    }


    #[derive(Debug)]
    pub enum FormatError<'a> {
        HexStrError(&'a str),
        Base64Error(&'a str),
    }

    impl Error for FormatError<'_> {}

    impl Display for FormatError<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                crate::convert::FormatError::HexStrError(msg) => write!(f, "{}", msg),
                crate::convert::FormatError::Base64Error(msg) => write!(f, "{}", msg),
            }
        }
    }

}

#[cfg(test)]
mod decode_tests {
    use crate::convert::*;

    #[test]
    fn test_hex_to_b64() {
        let mut input = "4d616e";
        let mut out = String::from("TWFu");

        assert_eq!(hex_str_to_b64(input).unwrap(), out);

        input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        out = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        assert_eq!(hex_str_to_b64(input).unwrap(), out);

        input = "68656c6c6f";
        out = String::from("aGVsbG8=");

        assert_eq!(hex_str_to_b64(input).unwrap(), out);

        input = "6161616161";
        out = String::from("YWFhYWE=");

        assert_eq!(hex_str_to_b64(input).unwrap(), out);

        input = "61";
        out = String::from("YQ==");

        assert_eq!(hex_str_to_b64(input).unwrap(), out);
    }
}
