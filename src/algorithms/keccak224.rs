// use crate::BinaryToTextEncoding;
// use crate::utils::encoders::{to_base64, to_base64url, to_binary_string, to_hex_string};

// #[derive(Clone)]
// pub struct Keccak224 {
//     state: [[u64; 5]; 5],
//     rate: usize,
//     capacity: usize,
//     delimited_suffix: u8,
//     buf: [u8; 200],
//     buf_filled: usize,
// }

// impl Keccak224 {
//     const RHO: [usize; 24] = [
//         1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14,
//         27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44
//     ];

//     const PI: [usize; 24] = [
//         10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4,
//         15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1
//     ];

//     const RC: [u64; 24] = [
//         0x0000000000000001,
//         0x0000000000008082,
//         0x800000000000808a,
//         0x8000000080008000,
//         0x000000000000808b,
//         0x0000000080000001,
//         0x8000000080008081,
//         0x8000000000008009,
//         0x000000000000008a,
//         0x0000000000000088,
//         0x0000000080008009,
//         0x000000008000000a,
//         0x000000008000808b,
//         0x800000000000008b,
//         0x8000000000008089,
//         0x8000000000008003,
//         0x8000000000008002,
//         0x8000000000000080,
//         0x000000000000800a,
//         0x800000008000000a,
//         0x8000000080008081,
//         0x8000000000008080,
//         0x0000000080000001,
//         0x8000000080008008
//     ];

//     pub fn new() -> Self {
//         Self {
//             state: [[0; 5]; 5],
//             rate: 144,
//             capacity: 224,
//             delimited_suffix: 0x06,
//             buf: [0; 200],
//             buf_filled: 0,
//         }
//     }

//     fn keccakf(&mut self) {
//         let mut a = self.state.clone();
//         let mut t: [u64; 5] = [0; 5];
//         let mut c: [u64; 5] = [0; 5];

//         for _ in 0..24 {
//             for i in 0..5 {
//                 c[i] = a[i][0] ^ a[i][1] ^ a[i][2] ^ a[i][3] ^ a[i][4];
//             }

//             for i in 0..5 {
//                 t[i] = c[(i + 4) % 5] ^ ((c[(i + 1) % 5]).rotate_left(1));
//                 for y in 0..5 {
//                     a[i][y] ^= t[i];
//                 }
//             }

//             let mut b = a.clone();

//             for y in 0..5 {
//                 for x in 0..5 {
//                     b[y][x] = a[(x + 3 * y) % 5][y].rotate_left(Self::RHO[5 * y + x] as u32);
//                 }
//             }

//             a = b.clone();

//             for y in 0..5 {
//                 for x in 0..5 {
//                     b[x][y] = a[x][y] ^ ((!a[(x + 1) % 5][y]) & a[(x + 2) % 5][y]);
//                 }
//             }

//             a = b.clone();

//             let mut rc_index = 0;

//             a[0][0] ^= Self::RC[rc_index % 24];
//             rc_index += 1;
//         }

//         self.state = a;
//     }

//     pub fn update(&mut self, input: &[u8]) {
//         for &byte in input.iter() {
//             self.buf[self.buf_filled] ^= byte;
//             self.buf_filled += 1;

//             if self.buf_filled == self.rate {
//                 self.keccakf();
//                 self.buf_filled = 0;
//             }
//         }
//     }

//     pub fn digest(&mut self, encoding: BinaryToTextEncoding) -> String {
//         self.buf[self.buf_filled] ^= self.delimited_suffix;
//         if ((self.delimited_suffix & 0x80) != 0) && (self.buf_filled == (self.rate - 1)) {
//             self.keccakf();
//         }
//         self.buf[self.rate - 1] ^= 0x80;
//         self.keccakf();

//         let mut output = Vec::new();
//         for i in 0..28 {
//             output.push((self.state[i / 8][i % 8] >> (8 * (i % 8))) as u8);
//         }

//         match encoding {
//             BinaryToTextEncoding::Base64 => to_base64(&output),
//             BinaryToTextEncoding::Base64Url => to_base64url(to_base64(&output)),
//             BinaryToTextEncoding::Hex => to_hex_string(&output),
//             BinaryToTextEncoding::Binary => to_binary_string(&output),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_keccak224() {
//         let mut keccak224 = Keccak224::new();
//         keccak224.update(b"hello world");
//         let result = keccak224.digest(BinaryToTextEncoding::Hex);
//         assert_eq!(result, "f875f5dda12d4293443cb84912e5cd1726d5a4909948d0172ab1f78f5564880b");
//     }
// }