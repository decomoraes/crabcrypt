// pub use digest::{self, Digest};

// use core::fmt;
// #[cfg(feature = "oid")]
// use digest::const_oid::{AssociatedOid, ObjectIdentifier};
// use digest::{
//     block_buffer::Eager,
//     consts::{U104, U136, U144, U168, U200, U28, U32, U48, U64, U72},
//     core_api::{
//         AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
//         ExtendableOutputCore, FixedOutputCore, OutputSizeUser, Reset, UpdateCore, XofReaderCore,
//         XofReaderCoreWrapper,
//     },
//     generic_array::typenum::Unsigned,
//     HashMarker, Output,
// };
// // --- MACROS ---
// #[macro_use]
// pub mod macros {
//     macro_rules! impl_sha3 {
//     (
//         $name:ident, $full_name:ident, $output_size:ident,
//         $rate:ident, $pad:expr, $alg_name:expr $(,)?
//     ) => {
//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $name {
//             state: Sha3State,
//         }

//         impl HashMarker for $name {}

//         impl BlockSizeUser for $name {
//             type BlockSize = $rate;
//         }

//         impl BufferKindUser for $name {
//             type BufferKind = Eager;
//         }

//         impl OutputSizeUser for $name {
//             type OutputSize = $output_size;
//         }

//         impl UpdateCore for $name {
//             #[inline]
//             fn update_blocks(&mut self, blocks: &[Block<Self>]) {
//                 for block in blocks {
//                     self.state.absorb_block(block)
//                 }
//             }
//         }

//         impl FixedOutputCore for $name {
//             #[inline]
//             fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
//                 let pos = buffer.get_pos();
//                 let block = buffer.pad_with_zeros();
//                 block[pos] = $pad;
//                 let n = block.len();
//                 block[n - 1] |= 0x80;

//                 self.state.absorb_block(block);

//                 self.state.as_bytes(out);
//             }
//         }

//         impl Default for $name {
//             #[inline]
//             fn default() -> Self {
//                 Self {
//                     state: Default::default(),
//                 }
//             }
//         }

//         impl Reset for $name {
//             #[inline]
//             fn reset(&mut self) {
//                 *self = Default::default();
//             }
//         }

//         impl AlgorithmName for $name {
//             fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(stringify!($full_name))
//             }
//         }

//         impl fmt::Debug for $name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(concat!(stringify!($name), " { ... }"))
//             }
//         }

//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         pub type $full_name = CoreWrapper<$name>;
//     };
//     (
//         $name:ident, $full_name:ident, $output_size:ident,
//         $rate:ident, $pad:expr, $alg_name:expr, $oid:literal $(,)?
//     ) => {
//         impl_sha3!($name, $full_name, $output_size, $rate, $pad, $alg_name);

//         #[cfg(feature = "oid")]
//         #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
//         impl AssociatedOid for $name {
//             const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
//         }
//     };
// }

//     macro_rules! impl_shake {
//     (
//         $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
//         $rate:ident, $pad:expr, $alg_name:expr $(,)?
//     ) => {
//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $name {
//             state: Sha3State,
//         }

//         impl HashMarker for $name {}

//         impl BlockSizeUser for $name {
//             type BlockSize = $rate;
//         }

//         impl BufferKindUser for $name {
//             type BufferKind = Eager;
//         }

//         impl UpdateCore for $name {
//             #[inline]
//             fn update_blocks(&mut self, blocks: &[Block<Self>]) {
//                 for block in blocks {
//                     self.state.absorb_block(block)
//                 }
//             }
//         }

//         impl ExtendableOutputCore for $name {
//             type ReaderCore = $reader;

//             #[inline]
//             fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
//                 let pos = buffer.get_pos();
//                 let block = buffer.pad_with_zeros();
//                 block[pos] = $pad;
//                 let n = block.len();
//                 block[n - 1] |= 0x80;

//                 self.state.absorb_block(block);
//                 $reader {
//                     state: self.state.clone(),
//                 }
//             }
//         }

//         impl Default for $name {
//             #[inline]
//             fn default() -> Self {
//                 Self {
//                     state: Default::default(),
//                 }
//             }
//         }

//         impl Reset for $name {
//             #[inline]
//             fn reset(&mut self) {
//                 *self = Default::default();
//             }
//         }

//         impl AlgorithmName for $name {
//             fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(stringify!($full_name))
//             }
//         }

//         impl fmt::Debug for $name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(concat!(stringify!($name), " { ... }"))
//             }
//         }

//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $reader {
//             state: Sha3State,
//         }

//         impl BlockSizeUser for $reader {
//             type BlockSize = $rate;
//         }

//         impl XofReaderCore for $reader {
//             #[inline]
//             fn read_block(&mut self) -> Block<Self> {
//                 let mut block = Block::<Self>::default();
//                 self.state.as_bytes(&mut block);
//                 self.state.permute();
//                 block
//             }
//         }

//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         pub type $full_name = CoreWrapper<$name>;

//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         pub type $reader_full = XofReaderCoreWrapper<$reader>;
//     };
//     (
//         $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
//         $rate:ident, $pad:expr, $alg_name:expr, $oid:literal $(,)?
//     ) => {
//         impl_shake!(
//             $name,
//             $full_name,
//             $reader,
//             $reader_full,
//             $rate,
//             $pad,
//             $alg_name
//         );

//         #[cfg(feature = "oid")]
//         #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
//         impl AssociatedOid for $name {
//             const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
//         }
//     };
// }

//     macro_rules! impl_turbo_shake {
//     (
//         $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
//         $rate:ident, $alg_name:expr $(,)?
//     ) => {
//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $name {
//             domain_separation: u8,
//             state: Sha3State,
//         }

//         impl $name {
//             /// Creates a new TurboSHAKE instance with the given domain separation.
//             /// Note that the domain separation needs to be a byte with a value in
//             /// the range [0x01, . . . , 0x7F]
//             pub fn new(domain_separation: u8) -> Self {
//                 assert!((0x01..=0x7F).contains(&domain_separation));
//                 Self {
//                     domain_separation,
//                     state: Sha3State::new(TURBO_SHAKE_ROUND_COUNT),
//                 }
//             }
//         }

//         impl HashMarker for $name {}

//         impl BlockSizeUser for $name {
//             type BlockSize = $rate;
//         }

//         impl BufferKindUser for $name {
//             type BufferKind = Eager;
//         }

//         impl UpdateCore for $name {
//             #[inline]
//             fn update_blocks(&mut self, blocks: &[Block<Self>]) {
//                 for block in blocks {
//                     self.state.absorb_block(block)
//                 }
//             }
//         }

//         impl ExtendableOutputCore for $name {
//             type ReaderCore = $reader;

//             #[inline]
//             fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
//                 let pos = buffer.get_pos();
//                 let block = buffer.pad_with_zeros();
//                 block[pos] = self.domain_separation;
//                 let n = block.len();
//                 block[n - 1] |= 0x80;

//                 self.state.absorb_block(block);
//                 $reader {
//                     state: self.state.clone(),
//                 }
//             }
//         }

//         impl Reset for $name {
//             #[inline]
//             fn reset(&mut self) {
//                 *self = Self::new(self.domain_separation);
//             }
//         }

//         impl AlgorithmName for $name {
//             fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(stringify!($full_name))
//             }
//         }

//         impl fmt::Debug for $name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(concat!(stringify!($name), " { ... }"))
//             }
//         }

//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $reader {
//             state: Sha3State,
//         }

//         impl BlockSizeUser for $reader {
//             type BlockSize = $rate;
//         }

//         impl XofReaderCore for $reader {
//             #[inline]
//             fn read_block(&mut self) -> Block<Self> {
//                 let mut block = Block::<Self>::default();
//                 self.state.as_bytes(&mut block);
//                 self.state.permute();
//                 block
//             }
//         }

//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         pub type $full_name = CoreWrapper<$name>;

//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         pub type $reader_full = XofReaderCoreWrapper<$reader>;
//     };
//     (
//         $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
//         $rate:ident, $alg_name:expr, $oid:literal $(,)?
//     ) => {
//         impl_turbo_shake!($name, $full_name, $reader, $reader_full, $rate, $alg_name);

//         #[cfg(feature = "oid")]
//         #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
//         impl AssociatedOid for $name {
//             const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
//         }
//     };
// }

//     macro_rules! impl_cshake {
//     (
//         $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
//         $rate:ident, $shake_pad:expr, $cshake_pad:expr, $alg_name:expr,
//     ) => {
//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $name {
//             padding: u8,
//             state: Sha3State,
//             #[cfg(feature = "reset")]
//             initial_state: Sha3State,
//         }

//         impl $name {
//             /// Creates a new CSHAKE instance with the given customization.
//             pub fn new(customization: &[u8]) -> Self {
//                 Self::new_with_function_name(&[], customization)
//             }

//             /// Creates a new CSHAKE instance with the given function name and customization.
//             /// Note that the function name is intended for use by NIST and should only be set to
//             /// values defined by NIST. You probably don't need to use this function.
//             pub fn new_with_function_name(function_name: &[u8], customization: &[u8]) -> Self {
//                 let mut state = Sha3State::default();
//                 if function_name.is_empty() && customization.is_empty() {
//                     return Self {
//                         padding: $shake_pad,
//                         state: state.clone(),
//                         #[cfg(feature = "reset")]
//                         initial_state: state,
//                     };
//                 }

//                 let mut buffer = Buffer::<Self>::default();
//                 let mut b = [0u8; 9];
//                 buffer.digest_blocks(left_encode($rate::to_u64(), &mut b), |blocks| {
//                     for block in blocks {
//                         state.absorb_block(block);
//                     }
//                 });
//                 buffer.digest_blocks(
//                     left_encode((function_name.len() * 8) as u64, &mut b),
//                     |blocks| {
//                         for block in blocks {
//                             state.absorb_block(block);
//                         }
//                     },
//                 );
//                 buffer.digest_blocks(function_name, |blocks| {
//                     for block in blocks {
//                         state.absorb_block(block);
//                     }
//                 });
//                 buffer.digest_blocks(
//                     left_encode((customization.len() * 8) as u64, &mut b),
//                     |blocks| {
//                         for block in blocks {
//                             state.absorb_block(block);
//                         }
//                     },
//                 );
//                 buffer.digest_blocks(customization, |blocks| {
//                     for block in blocks {
//                         state.absorb_block(block);
//                     }
//                 });
//                 state.absorb_block(buffer.pad_with_zeros());

//                 Self {
//                     padding: $cshake_pad,
//                     state: state.clone(),
//                     #[cfg(feature = "reset")]
//                     initial_state: state,
//                 }
//             }
//         }

//         impl HashMarker for $name {}

//         impl BlockSizeUser for $name {
//             type BlockSize = $rate;
//         }

//         impl BufferKindUser for $name {
//             type BufferKind = Eager;
//         }

//         impl UpdateCore for $name {
//             #[inline]
//             fn update_blocks(&mut self, blocks: &[Block<Self>]) {
//                 for block in blocks {
//                     self.state.absorb_block(block)
//                 }
//             }
//         }

//         impl ExtendableOutputCore for $name {
//             type ReaderCore = $reader;

//             #[inline]
//             fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
//                 let pos = buffer.get_pos();
//                 let block = buffer.pad_with_zeros();
//                 block[pos] = self.padding;
//                 let n = block.len();
//                 block[n - 1] |= 0x80;

//                 self.state.absorb_block(block);
//                 $reader {
//                     state: self.state.clone(),
//                 }
//             }
//         }

//         #[cfg(feature = "reset")]
//         impl Reset for $name {
//             #[inline]
//             fn reset(&mut self) {
//                 self.state = self.initial_state.clone();
//             }
//         }

//         impl AlgorithmName for $name {
//             fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(stringify!($full_name))
//             }
//         }

//         impl fmt::Debug for $name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 f.write_str(concat!(stringify!($name), " { ... }"))
//             }
//         }

//         #[doc = "Core "]
//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         #[derive(Clone)]
//         #[allow(non_camel_case_types)]
//         pub struct $reader {
//             state: Sha3State,
//         }

//         impl BlockSizeUser for $reader {
//             type BlockSize = $rate;
//         }

//         impl XofReaderCore for $reader {
//             #[inline]
//             fn read_block(&mut self) -> Block<Self> {
//                 let mut block = Block::<Self>::default();
//                 self.state.as_bytes(&mut block);
//                 self.state.permute();
//                 block
//             }
//         }

//         #[doc = $alg_name]
//         #[doc = " hasher state."]
//         pub type $full_name = CoreWrapper<$name>;

//         #[doc = $alg_name]
//         #[doc = " reader state."]
//         pub type $reader_full = XofReaderCoreWrapper<$reader>;
//     };
// }
// }
// // --- MACROS ---
// // --- STATE  ---
// pub mod state {
//     use core::convert::TryInto;
//     #[cfg(feature = "zeroize")]
//     use zeroize::{Zeroize, ZeroizeOnDrop};

//     const PLEN: usize = 25;
//     const DEFAULT_ROUND_COUNT: usize = 24;

//     #[derive(Clone)]
//     pub(crate) struct Sha3State {
//         pub state: [u64; PLEN],
//         round_count: usize,
//     }

//     impl Default for Sha3State {
//         fn default() -> Self {
//             Self {
//                 state: [0u64; PLEN],
//                 round_count: DEFAULT_ROUND_COUNT,
//             }
//         }
//     }

//     #[cfg(feature = "zeroize")]
//     impl Drop for Sha3State {
//         fn drop(&mut self) {
//             self.state.zeroize();
//         }
//     }

//     #[cfg(feature = "zeroize")]
//     impl ZeroizeOnDrop for Sha3State {}

//     impl Sha3State {
//         pub(crate) fn new(round_count: usize) -> Self {
//             Self {
//                 state: [0u64; PLEN],
//                 round_count,
//             }
//         }

//         #[inline(always)]
//         pub(crate) fn absorb_block(&mut self, block: &[u8]) {
//             debug_assert_eq!(block.len() % 8, 0);

//             for (b, s) in block.chunks_exact(8).zip(self.state.iter_mut()) {
//                 *s ^= u64::from_le_bytes(b.try_into().unwrap());
//             }

//             keccak::p1600(&mut self.state, self.round_count);
//         }

//         #[inline(always)]
//         pub(crate) fn as_bytes(&self, out: &mut [u8]) {
//             for (o, s) in out.chunks_mut(8).zip(self.state.iter()) {
//                 o.copy_from_slice(&s.to_le_bytes()[..o.len()]);
//             }
//         }

//         #[inline(always)]
//         pub(crate) fn permute(&mut self) {
//             keccak::p1600(&mut self.state, self.round_count);
//         }
//     }
// }
// // --- STATE  ---

// // #[macro_use]
// // mod macros;
// // mod state;

// use crate::state::Sha3State;

// // Paddings
// const KECCAK: u8 = 0x01;
// const SHA3: u8 = 0x06;
// const SHAKE: u8 = 0x1f;
// const CSHAKE: u8 = 0x4;

// // Round counts
// const TURBO_SHAKE_ROUND_COUNT: usize = 12;

// impl_sha3!(Keccak224Core, Keccak224, U28, U144, KECCAK, "Keccak-224");
// impl_sha3!(Keccak256Core, Keccak256, U32, U136, KECCAK, "Keccak-256");
// impl_sha3!(Keccak384Core, Keccak384, U48, U104, KECCAK, "Keccak-384");
// impl_sha3!(Keccak512Core, Keccak512, U64, U72, KECCAK, "Keccak-512");

// impl_sha3!(
//     Keccak256FullCore,
//     Keccak256Full,
//     U200,
//     U136,
//     KECCAK,
//     "SHA-3 CryptoNight variant",
// );

// impl_sha3!(
//     Sha3_224Core,
//     Sha3_224,
//     U28,
//     U144,
//     SHA3,
//     "SHA-3-224",
//     "2.16.840.1.101.3.4.2.7",
// );
// impl_sha3!(
//     Sha3_256Core,
//     Sha3_256,
//     U32,
//     U136,
//     SHA3,
//     "SHA-3-256",
//     "2.16.840.1.101.3.4.2.8",
// );
// impl_sha3!(
//     Sha3_384Core,
//     Sha3_384,
//     U48,
//     U104,
//     SHA3,
//     "SHA-3-384",
//     "2.16.840.1.101.3.4.2.9",
// );
// impl_sha3!(
//     Sha3_512Core,
//     Sha3_512,
//     U64,
//     U72,
//     SHA3,
//     "SHA-3-512",
//     "2.16.840.1.101.3.4.2.10",
// );

// impl_shake!(
//     Shake128Core,
//     Shake128,
//     Shake128ReaderCore,
//     Shake128Reader,
//     U168,
//     SHAKE,
//     "SHAKE128",
//     "2.16.840.1.101.3.4.2.11",
// );
// impl_shake!(
//     Shake256Core,
//     Shake256,
//     Shake256ReaderCore,
//     Shake256Reader,
//     U136,
//     SHAKE,
//     "SHAKE256",
//     "2.16.840.1.101.3.4.2.11",
// );

// impl_turbo_shake!(
//     TurboShake128Core,
//     TurboShake128,
//     TurboShake128ReaderCore,
//     TurboShake128Reader,
//     U168,
//     "TurboSHAKE128",
// );
// impl_turbo_shake!(
//     TurboShake256Core,
//     TurboShake256,
//     TurboShake256ReaderCore,
//     TurboShake256Reader,
//     U136,
//     "TurboSHAKE256",
// );

// impl_cshake!(
//     CShake128Core,
//     CShake128,
//     CShake128ReaderCore,
//     CShake128Reader,
//     U168,
//     SHAKE,
//     CSHAKE,
//     "CSHAKE128",
// );
// impl_cshake!(
//     CShake256Core,
//     CShake256,
//     CShake256ReaderCore,
//     CShake256Reader,
//     U136,
//     SHAKE,
//     CSHAKE,
//     "CSHAKE256",
// );

// #[inline(always)]
// pub(crate) fn left_encode(val: u64, b: &mut [u8; 9]) -> &[u8] {
//     b[1..].copy_from_slice(&val.to_be_bytes());
//     let i = b[1..8].iter().take_while(|&&a| a == 0).count();
//     b[i] = (8 - i) as u8;
//     &b[i..]
// }
