use crate::BinaryToTextEncoding;
use crate::utils::encoders::{to_base64, to_base64url, to_binary_string, to_hex_string};

#[derive(Copy, Clone)]
pub struct Sha1 {
    data: [u8; 64],
    datalen: usize,
    bitlen: u64,
    state: [u32; 5],
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            datalen: 0,
            bitlen: 0,
            state: [
                0x67452301,
                0xEFCDAB89,
                0x98BADCFE,
                0x10325476,
                0xC3D2E1F0,
            ],
        }
    }

    pub fn update(&mut self, data: &[u8]) -> Self {
        let mut i = 0usize;
        while i < data.len() {
            self.data[self.datalen] = data[i];
            self.datalen += 1;
            if self.datalen == 64 {
                self.transform();
                self.bitlen += 512;
                self.datalen = 0;
            }
            i += 1;
        }
        *self
    }

    pub fn digest(&mut self, encoding: BinaryToTextEncoding) -> String {
        let i = self.datalen;
        if self.datalen < 56 {
            self.data[i] = 0x80;
            for j in i+1..56 {
                self.data[j] = 0x00;
            }
        } else {
            self.data[i] = 0x80;
            for j in i+1..64 {
                self.data[j] = 0x00;
            }
            self.transform();
            for j in 0..56 {
                self.data[j] = 0x00;
            }
        }

        self.bitlen += self.datalen as u64 * 8;
        self.data[63] = self.bitlen as u8;
        self.data[62] = (self.bitlen >> 8) as u8;
        self.data[61] = (self.bitlen >> 16) as u8;
        self.data[60] = (self.bitlen >> 24) as u8;
        self.data[59] = (self.bitlen >> 32) as u8;
        self.data[58] = (self.bitlen >> 40) as u8;
        self.data[57] = (self.bitlen >> 48) as u8;
        self.data[56] = (self.bitlen >> 56) as u8;
        self.transform();

        let mut hash: [u8; 20] = [0; 20];
        for i in 0..4 {
            hash[i] = (self.state[0] >> (24 - i * 8)) as u8;
            hash[i + 4] = (self.state[1] >> (24 - i * 8)) as u8;
            hash[i + 8] = (self.state[2] >> (24 - i * 8)) as u8;
            hash[i + 12] = (self.state[3] >> (24 - i * 8)) as u8;
            hash[i + 16] = (self.state[4] >> (24 - i * 8)) as u8;
        }

        match encoding {
            BinaryToTextEncoding::Base64 => to_base64(&hash),
            BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&hash)),
            BinaryToTextEncoding::Hex => to_hex_string(&hash),
            BinaryToTextEncoding::Binary => to_binary_string(&hash),
        }
    }

    fn transform(&mut self) {
        let mut m: [u32; 80] = [0; 80];
        let mut a: u32;
        let mut b: u32;
        let mut c: u32;
        let mut d: u32;
        let mut e: u32;
        let mut f: u32;
        let mut k: u32;

        for i in 0..16 {
            let j = i * 4;
            m[i] = ((self.data[j] as u32) << 24) | ((self.data[j + 1] as u32) << 16)
                | ((self.data[j + 2] as u32) << 8) | (self.data[j + 3] as u32);
        }

        for i in 16..80 {
            m[i] = self.rotate_left(m[i - 3] ^ m[i - 8] ^ m[i - 14] ^ m[i - 16], 1);
        }

        a = self.state[0];
        b = self.state[1];
        c = self.state[2];
        d = self.state[3];
        e = self.state[4];

        for i in 0..80 {
            if i < 20 {
                f = (b & c) | (!b & d);
                k = 0x5A827999;
            } else if i < 40 {
                f = b ^ c ^ d;
                k = 0x6ED9EBA1;
            } else if i < 60 {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8F1BBCDC;
            } else {
                f = b ^ c ^ d;
                k = 0xCA62C1D6;
            }

            let temp = self.rotate_left(a, 5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(m[i]);
            e = d;
            d = c;
            c = self.rotate_left(b, 30);
            b = a;
            a = temp;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }

    fn rotate_left(&mut self, x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32 - n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        let sut = Sha1::new()
            .update(b"hello world")
            .digest(BinaryToTextEncoding::Hex);
        assert_eq!(sut, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }
}