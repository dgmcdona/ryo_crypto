pub mod xor {
    pub fn xor_buffers(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
        v1
            .into_iter()
            .zip(v2.into_iter())
            .map(|x| x.0 ^ x.1)
            .collect()
    }

    pub fn xor_single(input: Vec<u8>, key: u8) -> Vec<u8> {
        input
            .into_iter()
            .map(|x| x ^ key)
            .collect()
    }

    pub fn xor_crack_raw(input: Vec<u8>) -> u8 {

        let frequent_chars = vec!['e', 't', 'a', 'i', 'o', 'n', 's', 'h', 'r'];

        (0..=u8::MAX)
            .step_by(1)
            .map(|key| (key, input.iter()
                .filter(|x| frequent_chars.contains(&((*x ^ key) as char)))
                .count()))

            // find the (key, count) tuple with the highest count, and return the key.
            .fold((0, 0), |acc, z| if z.1 > acc.1 { return z } else { return acc})
            .0
    }
}

#[cfg(test)]
pub mod xor_tests {
    use crate::convert::ToBytes;
    use super::xor::*;
    use itertools::Itertools;

    #[test]
    fn test_xor() {
        let in1 = "1c0111001f010100061a024b53535009181c"
            .to_vec_u8().unwrap();

        let in2 = "686974207468652062756c6c277320657965"
            .to_vec_u8().unwrap();

        let xored = xor_buffers(in1, in2)
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .join("");

        let want = "746865206b696420646f6e277420706c6179";

        assert_eq!(want, xored);
    }

    #[test]
    fn test_xor_single() {
        let in1 = "20".to_vec_u8().unwrap();
        let key = 0x20;

        let result = xor_single(in1, key);
        assert_eq!(result[0], 0);
    }

    #[test]
    fn test_xor_single_2() {
        let in1 = "2F".to_vec_u8().unwrap();
        let key = 0x20;

        let result = xor_single(in1, key);
        assert_eq!(result[0], 0xf);
    }

    #[test]
    fn test_xor_single_multibyte_input() {
        let in1 = "2F2F2F".to_vec_u8().unwrap();
        let key = 0x20;

        let result = xor_single(in1, key);
        assert_eq!(result[0], 0xf);
        assert_eq!(result[1], 0xf);
        assert_eq!(result[2], 0xf);
    }

    #[test]
    fn test_xor_crack() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            .to_vec_u8().unwrap();

        let result = xor_crack_raw(input);
        assert_eq!(result, 88);

    }
}
