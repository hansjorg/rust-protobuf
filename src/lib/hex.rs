// hex encoder and decoder used by rust-protobuf unittests

use std::str;

fn decode_hex_digit(digit: char) -> u8 {
    if digit >= '0' && digit <= '9' {
        (digit - '0') as u8
    } else if digit >= 'a' && digit <= 'f' {
        (digit - 'a') as u8 + 10
    } else if digit >= 'A' && digit <= 'F' {
        (digit - 'A') as u8 + 10
    } else {
        fail!();
    }
}

pub fn decode_hex(hex: &str) -> ~[u8] {
    let mut r: ~[u8] = ~[];
    let mut pos = 0;
    loop {
        while pos < hex.char_len() && hex[pos] == ' ' as u8 {
            pos += 1;
        }
        if hex.char_len() - pos >= 2 {
            r.push((decode_hex_digit(hex.char_at(pos)) << 4) | decode_hex_digit(hex.char_at(pos + 1)));
            pos += 2;
            loop;
        }
        if pos == hex.char_len() {
            break;
        }
        fail!("pos = %ud", pos);
    }
    r
}

fn encode_hex_digit(digit: u8) -> char {
    if digit < 10 {
        '0' + (digit as char)
    } else if digit < 16 {
        'a' + ((digit - 10) as char)
    } else {
        fail!();
    }
}

fn encode_hex_byte(byte: u8) -> [char, ..2] {
    [encode_hex_digit(byte >> 4), encode_hex_digit(byte & 0x0Fu8)]
}

pub fn encode_hex(bytes: &[u8]) -> ~str {
    (do bytes.map |byte| {
        str::from_chars(encode_hex_byte(*byte))
    }).connect(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_hex() {
        assert_eq!(~[], decode_hex(""));
        assert_eq!(~[0x00], decode_hex("00"));
        assert_eq!(~[0xff], decode_hex("ff"));
        assert_eq!(~[0xab], decode_hex("AB"));
        assert_eq!(~[0xfa, 0x19], decode_hex("fa 19"));
    }

    #[test]
    fn test_encode_hex() {
        assert_eq!(~"", encode_hex([]));
        assert_eq!(~"00", encode_hex([0x00]));
        assert_eq!(~"ab", encode_hex([0xab]));
        assert_eq!(~"01 a2 1a fe", encode_hex([0x01, 0xa2, 0x1a, 0xfe]));
    }
}
