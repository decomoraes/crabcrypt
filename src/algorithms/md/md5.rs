use crate::BinaryToTextEncoding;
use crate::utils::encoders::{to_base64, to_base64url, to_binary_string, to_hex_string};

#[derive(Copy, Clone)]
pub struct Md5 {
    data: [u8; 64],
    datalen: usize,
    bitlen: u64,
    state: [u32; 4],
}

impl Md5 {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            datalen: 0,
            bitlen: 0,
            state: [
                0x67452301,
                0xefcdab89,
                0x98badcfe,
                0x10325476,
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
        self.data[56] = self.bitlen as u8;
        self.data[57] = (self.bitlen >> 8) as u8;
        self.data[58] = (self.bitlen >> 16) as u8;
        self.data[59] = (self.bitlen >> 24) as u8;
        self.data[60] = (self.bitlen >> 32) as u8;
        self.data[61] = (self.bitlen >> 40) as u8;
        self.data[62] = (self.bitlen >> 48) as u8;
        self.data[63] = (self.bitlen >> 56) as u8;
        self.transform();

        // self.bitlen += self.datalen as u64 * 8;
        // self.data[63] = self.bitlen as u8;
        // self.data[62] = (self.bitlen >> 8) as u8;
        // self.data[61] = (self.bitlen >> 16) as u8;
        // self.data[60] = (self.bitlen >> 24) as u8;
        // self.data[59] = (self.bitlen >> 32) as u8;
        // self.data[58] = (self.bitlen >> 40) as u8;
        // self.data[57] = (self.bitlen >> 48) as u8;
        // self.data[56] = (self.bitlen >> 56) as u8;
        // self.transform();

        let mut hash: [u8; 16] = [0; 16];
        for i in 0..4 {
            hash[i] = (self.state[0] >> (8 * i)) as u8;
            hash[i + 4] = (self.state[1] >> (8 * i)) as u8;
            hash[i + 8] = (self.state[2] >> (8 * i)) as u8;
            hash[i + 12] = (self.state[3] >> (8 * i)) as u8;
        }

        match encoding {
            BinaryToTextEncoding::Base64 => to_base64(&hash),
            BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&hash)),
            BinaryToTextEncoding::Hex => to_hex_string(&hash),
            BinaryToTextEncoding::Binary => to_binary_string(&hash),
        }
    }

    fn transform(&mut self) {
        // MD5 magic constants
        let s: [u32; 64] = [
            7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
            5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
            4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
            6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
        ];
        let k: [u32; 64] = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
            0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
            0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
            0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
            0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
            0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
            0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
            0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
            0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
        ];

        let mut m: [u32; 16] = [0; 16];
        for i in 0..16 {
            m[i] = ((self.data[i*4 + 3] as u32) << 24) | ((self.data[i*4 + 2] as u32) << 16)
                | ((self.data[i*4 + 1] as u32) << 8) | self.data[i*4] as u32;
        }

        // for i in 0..16 {
        //     m[i] = ((self.data[i*4] as u32) << 24) | ((self.data[i*4 + 1] as u32) << 16)
        //         | ((self.data[i*4 + 2] as u32) << 8) | self.data[i*4 + 3] as u32;
        // }

        let (mut a, mut b, mut c, mut d) = (self.state[0], self.state[1], self.state[2], self.state[3]);

        for i in 0..64 {
            let (f, g) = if i < 16 {
                ((b & c) | (!b & d), i)
            } else if i < 32 {
                ((d & b) | (!d & c), (5*i + 1)%16)
            } else if i < 48 {
                (b ^ c ^ d, (3*i + 5)%16)
            } else {
                (c ^ (b | !d), (7*i)%16)
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(self.rotate_left(a.wrapping_add(f).wrapping_add(k[i]).wrapping_add(m[g]), s[i]));
            a = temp;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn rotate_left(&self, x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32 - n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let sut = Md5::new()
            .update(b"hello world")
            .digest(BinaryToTextEncoding::Hex);
        assert_eq!(sut, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}