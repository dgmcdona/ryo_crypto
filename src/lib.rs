pub mod crypt;

pub mod convert {
    use core::fmt;
    use itertools::Itertools;
    use std::error::Error;
    use std::fmt::{Debug, Display};
    use std::u8;

    static B64CHARS: &'static [char] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '0', '+', '/',
    ];

    pub fn hex_str_to_b64(input: &str) -> Result<String, Box<dyn Error>> {
        let bytes = input.to_vec_u8()?;
        Ok(bytes_to_b64(bytes))
    }

    fn chunk_to_b64(input: &[u8]) -> String {
        let mut chrs = String::new();

        chrs.push(B64CHARS[(input[0] >> 2) as usize]);

        if input.len() >= 2 {
            let v = ((input[0] & 0x3) << 4) | ((input[1] & 0xf0) >> 4);
            chrs.push(B64CHARS[v as usize]);
        } else {
            chrs.push(B64CHARS[((input[0] & 0x3) << 4) as usize]);
            chrs.push('=');
            chrs.push('=');
            return chrs;
        }

        if input.len() >= 3 {
            let v = ((input[1] & 0xf) << 2) | (((input[2] & 0xc0) >> 6) & 0xf);
            chrs.push(B64CHARS[v as usize]);
        } else {
            chrs.push(B64CHARS[((input[1] & 0xf) << 2) as usize]);
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
            .map(|i| chunk_to_b64(i))
            .collect()
    }

    // Implement this trait to specify a conversion between your type and Vec<u8>.
    pub trait ToBytes {
        fn to_vec_u8(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    }


    impl ToBytes for &str {
        fn to_vec_u8(&self) -> Result<Vec<u8>, Box<dyn Error>> {
            match self
                .chars()
                .chunks(2)
                .into_iter()
                .map(|x| u8::from_str_radix(x.collect::<String>().as_str(), 16))
                .collect() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Box::new(e)),
                }
        }
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
mod convert_tests {

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
