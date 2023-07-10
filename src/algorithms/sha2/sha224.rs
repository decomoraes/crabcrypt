use crate::BinaryToTextEncoding;
use crate::utils::encoders::{to_base64, to_base64url, to_binary_string, to_hex_string};

#[derive(Copy, Clone)]
pub struct Sha224 {
    data: [u8; 64],
    datalen: usize,
    bitlen: u64,
    state: [u32; 8],
}

impl Sha224 {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            datalen: 0,
            bitlen: 0,
            state: [
                0xc1059ed8,
                0x367cd507,
                0x3070dd17,
                0xf70e5939,
                0xffc00b31,
                0x68581511,
                0x64f98fa7,
                0xbefa4fa4,
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

        let mut hash: [u8; 28] = [0; 28];
        for i in 0..4 {
            hash[i] = (self.state[0] >> (24 - i * 8)) as u8;
            hash[i + 4] = (self.state[1] >> (24 - i * 8)) as u8;
            hash[i + 8] = (self.state[2] >> (24 - i * 8)) as u8;
            hash[i + 12] = (self.state[3] >> (24 - i * 8)) as u8;
            hash[i + 16] = (self.state[4] >> (24 - i * 8)) as u8;
            hash[i + 20] = (self.state[5] >> (24 - i * 8)) as u8;
            hash[i + 24] = (self.state[6] >> (24 - i * 8)) as u8;
        }

        match encoding {
            BinaryToTextEncoding::Base64 => to_base64(&hash),
            BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&hash)),
            BinaryToTextEncoding::Hex => to_hex_string(&hash),
            BinaryToTextEncoding::Binary => to_binary_string(&hash),
        }
    }

    fn transform(&mut self) {
        let k: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
        ];
        let mut m: [u32; 64] = [0; 64];
        let mut a: u32;
        let mut b: u32;
        let mut c: u32;
        let mut d: u32;
        let mut e: u32;
        let mut f: u32;
        let mut g: u32;
        let mut h: u32;
        let mut t1: u32;
        let mut t2: u32;

        for i in 0..16 {
            let j = i * 4;
            m[i] = ((self.data[j] as u32) << 24) | ((self.data[j + 1] as u32) << 16)
                | ((self.data[j + 2] as u32) << 8) | (self.data[j + 3] as u32);
        }

        for i in 16..64 {
            let s0 = self.rotate_right(m[i - 15], 7) ^ self.rotate_right(m[i - 15], 18) ^ (m[i - 15] >> 3);
            let s1 = self.rotate_right(m[i - 2], 17) ^ self.rotate_right(m[i - 2], 19) ^ (m[i - 2] >> 10);
            m[i] = m[i - 16].wrapping_add(s0).wrapping_add(m[i - 7]).wrapping_add(s1);
        }

        a = self.state[0];
        b = self.state[1];
        c = self.state[2];
        d = self.state[3];
        e = self.state[4];
        f = self.state[5];
        g = self.state[6];
        h = self.state[7];

        for i in 0..64 {
            let s1 = self.rotate_right(e, 6) ^ self.rotate_right(e, 11) ^ self.rotate_right(e, 25);
            let ch = (e & f) ^ (!e & g);
            t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(m[i]);
            let s0 = self.rotate_right(a, 2) ^ self.rotate_right(a, 13) ^ self.rotate_right(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            t2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }

    fn rotate_right(&mut self, x: u32, n: u32) -> u32 {
        (x >> n) | (x << (32 - n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha224() {
        let sut = Sha224::new()
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(BinaryToTextEncoding::Hex);
        assert_eq!(sut, "730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525".to_string());
    }
}