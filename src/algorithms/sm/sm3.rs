use crate::BinaryToTextEncoding;
use crate::utils::encoders::{to_base32, to_base64, to_base64url, to_binary_string, to_hex_string};

use core::convert::TryFrom;

const INITIAL_STATE: [u32; 8] = [
    0x7380_166f,
    0x4914_b2b9,
    0x1724_42d7,
    0xda8a_0600,
    0xa96f_30bc,
    0x1631_38aa,
    0xe38d_ee4d,
    0xb0fb_0e4e,
];

#[derive(Copy, Clone)]
pub struct Sm3 {
    buffer: [u8; Self::BLOCK_LEN],
    state: [u32; 8],
    len: u64, // in bytes
    offset: usize,
}

impl Sm3 {
    pub const BLOCK_LEN: usize = 64;
    pub const DIGEST_LEN: usize = 32;

    const BLOCK_LEN_BITS: u64 = Self::BLOCK_LEN as u64 * 8;
    const MLEN_SIZE: usize = core::mem::size_of::<u64>();
    const MLEN_SIZE_BITS: u64 = Self::MLEN_SIZE as u64 * 8;
    const MAX_PAD_LEN: usize = Self::BLOCK_LEN + Self::MLEN_SIZE as usize;

    pub fn new() -> Self {
        Self {
            buffer: [0u8; Self::BLOCK_LEN],
            state: INITIAL_STATE,
            len: 0,
            offset: 0usize,
        }
    }

    pub fn update(&mut self, data: &[u8]) -> Self {
        let mut i = 0usize;
        while i < data.len() {
            if self.offset < Self::BLOCK_LEN {
                self.buffer[self.offset] = data[i];
                self.offset += 1;
                i += 1;
            }

            if self.offset == Self::BLOCK_LEN {
                transform(&mut self.state, &self.buffer);
                self.offset = 0;
                self.len += Self::BLOCK_LEN as u64;
            }
        }

        *self
    }

    pub fn digest(mut self, encoding: BinaryToTextEncoding) -> String {
        let mlen = self.len + self.offset as u64; // in bytes
        let mlen_bits = mlen * 8; // in bits

        // pad len, in bits
        let plen_bits = Self::BLOCK_LEN_BITS
            - (mlen_bits + Self::MLEN_SIZE_BITS + 1) % Self::BLOCK_LEN_BITS
            + 1;
        // pad len, in bytes
        let plen = plen_bits / 8;
        debug_assert_eq!(plen_bits % 8, 0);
        debug_assert!(plen > 1);
        debug_assert_eq!(
            (mlen + plen + Self::MLEN_SIZE as u64) % Self::BLOCK_LEN as u64,
            0
        );

        // NOTE: MAX_PAD_LEN 是一个很小的数字，所以这里可以安全的 unwrap.
        let plen = usize::try_from(plen).unwrap();

        let mut padding: [u8; Self::MAX_PAD_LEN] = [0u8; Self::MAX_PAD_LEN];
        padding[0] = 0x80;

        let mlen_octets: [u8; Self::MLEN_SIZE] = mlen_bits.to_be_bytes();
        padding[plen..plen + Self::MLEN_SIZE].copy_from_slice(&mlen_octets);

        let data = &padding[..plen + Self::MLEN_SIZE];
        self.update(data);

        // NOTE: 数据填充完毕后，此时已经处理的消息应该是 BLOCK_LEN 的倍数，因此，offset 此时已被清零。
        debug_assert_eq!(self.offset, 0);

        let mut hash = [0u8; Self::DIGEST_LEN];
        hash[0..4].copy_from_slice(&self.state[0].to_be_bytes());
        hash[4..8].copy_from_slice(&self.state[1].to_be_bytes());
        hash[8..12].copy_from_slice(&self.state[2].to_be_bytes());
        hash[12..16].copy_from_slice(&self.state[3].to_be_bytes());
        hash[16..20].copy_from_slice(&self.state[4].to_be_bytes());
        hash[20..24].copy_from_slice(&self.state[5].to_be_bytes());
        hash[24..28].copy_from_slice(&self.state[6].to_be_bytes());
        hash[28..32].copy_from_slice(&self.state[7].to_be_bytes());

        match encoding {
            BinaryToTextEncoding::Base32 => to_base32(&hash),
            BinaryToTextEncoding::Base64 => to_base64(&hash),
            BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&hash)),
            BinaryToTextEncoding::Hex => to_hex_string(&hash),
            BinaryToTextEncoding::Binary => to_binary_string(&hash),
        }
    }
}

#[inline(always)]
fn ff0(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
fn ff1(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

#[inline(always)]
fn gg0(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
fn gg1(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

#[inline(always)]
fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

#[inline]
fn transform(state: &mut [u32; 8], block: &[u8; Sm3::BLOCK_LEN]) {
    // get expend
    let mut w: [u32; 68] = [0; 68];
    let mut w1: [u32; 64] = [0; 64];

    for i in 0..16 {
        let a = block[i * 4 + 0];
        let b = block[i * 4 + 1];
        let c = block[i * 4 + 2];
        let d = block[i * 4 + 3];
        w[i] = u32::from_be_bytes([a, b, c, d]);
    }

    for i in 16..68 {
        w[i] = p1(w[i - 16] ^ w[i - 9] ^ w[i - 3].rotate_left(15))
            ^ w[i - 13].rotate_left(7)
            ^ w[i - 6];
    }

    for i in 0..Sm3::BLOCK_LEN {
        w1[i] = w[i] ^ w[i + 4];
    }

    let mut ra = state[0];
    let mut rb = state[1];
    let mut rc = state[2];
    let mut rd = state[3];
    let mut re = state[4];
    let mut rf = state[5];
    let mut rg = state[6];
    let mut rh = state[7];
    let mut ss1: u32;
    let mut ss2: u32;
    let mut tt1: u32;
    let mut tt2: u32;

    for i in 0..16 {
        ss1 = ra
            .rotate_left(12)
            .wrapping_add(re)
            .wrapping_add(0x79cc_4519u32.rotate_left(i as u32))
            .rotate_left(7);
        ss2 = ss1 ^ ra.rotate_left(12);
        tt1 = ff0(ra, rb, rc)
            .wrapping_add(rd)
            .wrapping_add(ss2)
            .wrapping_add(w1[i]);
        tt2 = gg0(re, rf, rg)
            .wrapping_add(rh)
            .wrapping_add(ss1)
            .wrapping_add(w[i]);
        rd = rc;
        rc = rb.rotate_left(9);
        rb = ra;
        ra = tt1;
        rh = rg;
        rg = rf.rotate_left(19);
        rf = re;
        re = p0(tt2);
    }

    for i in 16..64 {
        ss1 = ra
            .rotate_left(12)
            .wrapping_add(re)
            .wrapping_add(0x7a87_9d8au32.rotate_left(i as u32))
            .rotate_left(7);
        ss2 = ss1 ^ ra.rotate_left(12);
        tt1 = ff1(ra, rb, rc)
            .wrapping_add(rd)
            .wrapping_add(ss2)
            .wrapping_add(w1[i]);
        tt2 = gg1(re, rf, rg)
            .wrapping_add(rh)
            .wrapping_add(ss1)
            .wrapping_add(w[i]);
        rd = rc;
        rc = rb.rotate_left(9);
        rb = ra;
        ra = tt1;
        rh = rg;
        rg = rf.rotate_left(19);
        rf = re;
        re = p0(tt2);
    }

    state[0] ^= ra;
    state[1] ^= rb;
    state[2] ^= rc;
    state[3] ^= rd;
    state[4] ^= re;
    state[5] ^= rf;
    state[6] ^= rg;
    state[7] ^= rh;
}

#[test]
fn test_sm3() {
    let sut = Sm3::new()
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(BinaryToTextEncoding::Hex);
    assert_eq!(sut, "5fdfe814b8573ca021983970fc79b2218c9570369b4859684e2e4c3fc76cb8ea");
}