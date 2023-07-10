use crate::BinaryToTextEncoding;
use crate::utils::encoders::{to_base32, to_base64, to_base64url, to_binary_string, to_hex_string};
use std::convert::TryInto;


#[derive(Copy, Clone)]
pub struct Sha512 {
    data: [u8; 128],
    datalen: usize,
    bitlen: u128,
    state: [u64; 8],
}

impl Sha512 {
    pub fn new() -> Self {
        Self {
            data: [0; 128],
            datalen: 0,
            bitlen: 0,
            state: [
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
                0x510e527fade682d1,
                0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b,
                0x5be0cd19137e2179,
            ],
        }
    }

    pub fn update(&mut self, data: &[u8]) -> Self {
        let mut i = 0usize;
        while i < data.len() {
            self.data[self.datalen] = data[i];
            self.datalen += 1;
            if self.datalen == 128 {
                self.transform();
                self.bitlen += 1024;
                self.datalen = 0;
            }
            i += 1;
        }
        *self
    }

    // rest of the implementation stays the same as your Sha384
    // except for the digest function

    pub fn digest(&mut self, encoding: BinaryToTextEncoding) -> String {

        let i = self.datalen;
        if self.datalen < 112 {
            self.data[i] = 0x80;
            for j in i+1..112 {
                self.data[j] = 0x00;
            }
        } else {
            self.data[i] = 0x80;
            for j in i+1..128 {
                self.data[j] = 0x00;
            }
            self.transform();
            for j in 0..112 {
                self.data[j] = 0x00;
            }
        }

        self.bitlen += self.datalen as u128 * 8;
        self.data[127] = self.bitlen as u8;
        self.data[126] = (self.bitlen >> 8) as u8;
        self.data[125] = (self.bitlen >> 16) as u8;
        self.data[124] = (self.bitlen >> 24) as u8;
        self.data[123] = (self.bitlen >> 32) as u8;
        self.data[122] = (self.bitlen >> 40) as u8;
        self.data[121] = (self.bitlen >> 48) as u8;
        self.data[120] = (self.bitlen >> 56) as u8;
        self.data[119] = (self.bitlen >> 64) as u8;
        self.data[118] = (self.bitlen >> 72) as u8;
        self.data[117] = (self.bitlen >> 80) as u8;
        self.data[116] = (self.bitlen >> 88) as u8;
        self.data[115] = (self.bitlen >> 96) as u8;
        self.data[114] = (self.bitlen >> 104) as u8;
        self.data[113] = (self.bitlen >> 112) as u8;
        self.data[112] = (self.bitlen >> 120) as u8;
        self.transform();
        // the rest of the method is the same until here

        let mut hash: [u8; 64] = [0; 64];
        for i in 0..8 {
            hash[i * 8] = (self.state[i] >> 56) as u8;
            hash[i * 8 + 1] = (self.state[i] >> 48) as u8;
            hash[i * 8 + 2] = (self.state[i] >> 40) as u8;
            hash[i * 8 + 3] = (self.state[i] >> 32) as u8;
            hash[i * 8 + 4] = (self.state[i] >> 24) as u8;
            hash[i * 8 + 5] = (self.state[i] >> 16) as u8;
            hash[i * 8 + 6] = (self.state[i] >> 8) as u8;
            hash[i * 8 + 7] = self.state[i] as u8;
        }

        match encoding {
            BinaryToTextEncoding::Base32 => to_base32(&hash),
            BinaryToTextEncoding::Base64 => to_base64(&hash),
            BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&hash)),
            BinaryToTextEncoding::Hex => to_hex_string(&hash),
            BinaryToTextEncoding::Binary => to_binary_string(&hash),
        }
    }


    fn transform(&mut self) {

        const K: [u64; 80] = [
            0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
            0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
            0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
            0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
            0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
            0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
            0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
            0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
            0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
            0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
            0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
            0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
            0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
            0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
            0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
            0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
            0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
            0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
            0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
            0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
        ];

        fn ch(x: u64, y: u64, z: u64) -> u64 {
            (x & y) ^ (!x & z)
        }

        fn maj(x: u64, y: u64, z: u64) -> u64 {
            (x & y) ^ (x & z) ^ (y & z)
        }

        fn rotr(x: u64, n: u64) -> u64 {
            (x >> n) | (x << (64 - n))
        }

        fn shr(x: u64, n: u64) -> u64 {
            x >> n
        }

        fn sigma0(x: u64) -> u64 {
            rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
        }

        fn sigma1(x: u64) -> u64 {
            rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
        }

        fn gamma0(x: u64) -> u64 {
            rotr(x, 1) ^ rotr(x, 8) ^ shr(x, 7)
        }

        fn gamma1(x: u64) -> u64 {
            rotr(x, 19) ^ rotr(x, 61) ^ shr(x, 6)
        }

        let mut w: [u64; 80] = [0; 80];

        // Divide o bloco de dados em 16 palavras de 64 bits
        for i in 0..16 {
            w[i] = u64::from_be_bytes(self.data[i*8..i*8+8].try_into().unwrap());
        }

        // Expandindo as palavras para w[16]..w[63]
        for i in 16..80 {
            w[i] = gamma1(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(gamma0(w[i - 15]))
                .wrapping_add(w[i - 16]);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        // Compressão principal
        for i in 0..80 {
            let t1 = h
                .wrapping_add(sigma1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let t2 = sigma0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Adiciona o hash deste bloco ao hash total
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha512() {
        let sut = Sha512::new()
            .update(b"hello world")
            .digest(BinaryToTextEncoding::Hex);
        assert_eq!(sut, "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f");
    }
}