pub fn to_hex_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

pub fn to_base32(bytes: &[u8]) -> String {
    let chars = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '2', '3', '4', '5', '6', '7',
    ];

    let mut result = String::new();

    let mut buffer = 0;
    let mut buffer_length = 0;

    for &byte in bytes {
        buffer = (buffer << 8) | (byte as u32);
        buffer_length += 8;

        while buffer_length >= 5 {
            buffer_length -= 5;
            result.push(chars[((buffer >> buffer_length) & 0x1f) as usize]);
        }
    }

    if buffer_length > 0 {
        result.push(chars[((buffer << (5 - buffer_length)) & 0x1f) as usize]);
    }

    result
}

#[allow(dead_code)]
pub fn to_base36(bytes: &[u8]) -> String {
    const TABLE: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut res = String::new();

    // Convert the bytes to a big integer.
    let mut big_num = bytes.iter().fold(0u128, |acc, b| (acc << 8) | (*b as u128));

    // While the big integer is not 0, take its remainder when divided by 36, find the
    // corresponding character, and add it to the start of the output string.
    while big_num > 0 {
        let remainder = (big_num % 36) as usize;
        big_num /= 36;
        res.insert(0, TABLE[remainder] as char);
    }

    // If the string is empty, that means the input was empty or all zeros. In either case,
    // the correct output is "0".
    if res.is_empty() {
        res.push('0');
    }

    res
}

pub fn to_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut res = String::new();

    let mut iter = bytes.iter();

    while let (Some(&b1), b2, b3) = (iter.next(), iter.next(), iter.next()) {
        res.push(TABLE[(b1 >> 2) as usize] as char);
        res.push(TABLE[((b1 & 0x3) << 4 | (b2.map(|b| *b).unwrap_or(0) & 0xf0) >> 4) as usize] as char);

        match (b2, b3) {
            (Some(&b2), Some(&b3)) => {
                res.push(TABLE[((b2 & 0xf) << 2 | (b3 & 0xc0) >> 6) as usize] as char);
                res.push(TABLE[(b3 & 0x3f) as usize] as char);
            },
            (Some(&b2), None) => {
                res.push(TABLE[((b2 & 0xf) << 2) as usize] as char);
                res.push('=');
            },
            (None, None) => {
                res.push_str("==");
            },
            _ => (),
        }
    }

    res
}

pub fn to_base64url(base64: String) -> String {
    base64.replace("+", "-").replace("/", "_").trim_end_matches('=').to_string()
}

pub fn to_binary_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        s.push_str(&format!("{:08b}", byte));
    }
    s
}