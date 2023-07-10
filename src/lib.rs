use crate::algorithms::md::md2::Md2;
use crate::algorithms::md::md4::Md4;
use crate::algorithms::md::md5::Md5;
use crate::algorithms::sha1::sha1::Sha1;
use crate::algorithms::sha2::sha224::Sha224;
use crate::algorithms::sha2::sha256::Sha256;
use crate::algorithms::sha2::sha384::Sha384;
use crate::algorithms::sha2::sha512::Sha512;
use crate::algorithms::sm::sm3::Sm3;
use crate::BinaryToTextEncoding::Hex;

/// Enumeration of cipher options.
#[allow(dead_code)]
enum Ciphers {
    Aes128Cbc,
    Aes128Ecb,
    Aes192Cbc,
    Aes192Ecb,
    Aes256Cbc,
    Aes256Ecb,
    Base64,
    Bf,
    BfCbc,
    BfCfb,
    BfEcb,
    BfOfb,
    Camellia128Cbc,
    Camellia128Ecb,
    Camellia192Cbc,
    Camellia192Ecb,
    Camellia256Cbc,
    Camellia256Ecb,
    Cast,
    CastCbc,
    Cast5Cbc,
    Cast5Cfb,
    Cast5Ecb,
    Cast5Ofb,
    Chacha,
    Des,
    DesCbc,
    DesCfb,
    DesEcb,
    DesEde,
    DesEdeCbc,
    DesEdeCfb,
    DesEdeOfb,
    DesEde3,
    DesEde3Cbc,
    DesEde3Cfb,
    DesEde3Ofb,
    DesOfb,
    Des3,
    Desx,
    Rc2,
    Rc240Cbc,
    Rc264Cbc,
    Rc2Cbc,
    Rc2Cfb,
    Rc2Ecb,
    Rc2Ofb,
    Rc4,
    Rc440,
}

mod algorithms;
mod sig;
mod utils;

/// Enumeration of encoding options for binary data to text.
#[derive(Copy, Clone)]
pub enum BinaryToTextEncoding {
    Base32,
    Base64,
    Base64Url,
    Hex,
    Binary,
}

/// Enumeration of character encoding options.
#[allow(dead_code)]
pub enum CharacterEncoding {
    Utf8,
    Utf16le,
    Latin1,
}

/// Enumeration of legacy character encoding options.
pub enum LegacyCharacterEncoding {
    Ascii,
    Binary,
    Ucs2,
}

/// Enumeration of possible encoding types.
pub enum Encoding {
    BinaryToTextEncoding(BinaryToTextEncoding),
    CharacterEncoding(CharacterEncoding),
    LegacyCharacterEncoding(LegacyCharacterEncoding),
}

/// Enumeration of ECDH key format options.
pub enum ECDHKeyFormat {
    Compressed,
    Uncompressed,
    Hybrid,
}

/// Enumeration of algorithm options.
#[derive(Copy, Clone)]
pub enum Algorithms {
    // GostMac,
    Md2, // ok
    Md4, // ok
    Md5, // ok
    // MdGost94,
    // Ripemd160,
    Sha1,   // ok
    Sha224, // ok
    Sha256, // ok
    Sha384, // ok
    Sha512, // ok
    Sm3,    // ok
            // Sha3_224,
            // Sha3_256,
            // Sha3_384,
            // Sha3_512,
            // Streebog256,
            // Streebog512,
            // Whirlpool,
}

/// Structure for the hash operation.
#[derive(Copy, Clone)]
pub struct Hash {
    pub algorithm: Algorithms,
    // GostMac,
    pub md2: Option<Md2>,
    pub md4: Option<Md4>,
    pub md5: Option<Md5>,
    // MdGost94,
    // Ripemd160,
    pub sha1: Option<Sha1>,
    pub sha224: Option<Sha224>,
    pub sha256: Option<Sha256>,
    pub sha384: Option<Sha384>,
    pub sha512: Option<Sha512>,
    pub sm3: Option<Sm3>,
    // Streebog256,
    // Streebog512,
    // Whirlpool,
}

impl Hash {
    /// Function to initialize the hashing algorithm.
    pub fn create(algorithm: Algorithms) -> Hash {
        Hash {
            algorithm,
            md2: match &algorithm {
                Algorithms::Md2 => Some(Md2::new()),
                _ => None,
            },
            md4: match &algorithm {
                Algorithms::Md4 => Some(Md4::new()),
                _ => None,
            },
            md5: match &algorithm {
                Algorithms::Md5 => Some(Md5::new()),
                _ => None,
            },
            sha1: match &algorithm {
                Algorithms::Sha1 => Some(Sha1::new()),
                _ => None,
            },
            sha224: match &algorithm {
                Algorithms::Sha224 => Some(Sha224::new()),
                _ => None,
            },
            sha256: match &algorithm {
                Algorithms::Sha256 => Some(Sha256::new()),
                _ => None,
            },
            sha384: match &algorithm {
                Algorithms::Sha384 => Some(Sha384::new()),
                _ => None,
            },
            sha512: match &algorithm {
                Algorithms::Sha512 => Some(Sha512::new()),
                _ => None,
            },
            sm3: match &algorithm {
                Algorithms::Sm3 => Some(Sm3::new()),
                _ => None,
            },
        }
    }

    /// Function to update the data to be hashed.
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        match self.algorithm {
            Algorithms::Md2 => {
                if let Some(md2) = self.md2.as_mut() {
                    md2.update(data);
                }
            }
            Algorithms::Md4 => {
                if let Some(md4) = self.md4.as_mut() {
                    md4.update(data);
                }
            }
            Algorithms::Md5 => {
                if let Some(md5) = self.md5.as_mut() {
                    md5.update(data);
                }
            }
            Algorithms::Sha1 => {
                if let Some(sha1) = self.sha1.as_mut() {
                    sha1.update(data);
                }
            }
            Algorithms::Sha224 => {
                if let Some(sha224) = self.sha224.as_mut() {
                    sha224.update(data);
                }
            }
            Algorithms::Sha256 => {
                if let Some(sha256) = self.sha256.as_mut() {
                    sha256.update(data);
                }
                // else {
                //     let mut sha256 = Sha256::new();
                //     sha256.update(data);
                //     self.sha256 = Some(sha256);
                // }
            }
            Algorithms::Sha384 => {
                if let Some(sha384) = self.sha384.as_mut() {
                    sha384.update(data);
                }
            }
            Algorithms::Sha512 => {
                if let Some(sha512) = self.sha512.as_mut() {
                    sha512.update(data);
                }
            }
            Algorithms::Sm3 => {
                if let Some(sm3) = self.sm3.as_mut() {
                    sm3.update(data);
                }
            }
        }
        self
    }

    /// Function to produce the digest of the hashed data.
    pub fn digest(&mut self, encoding: BinaryToTextEncoding) -> String {
        match self.algorithm {
            Algorithms::Md2 => self.md2.unwrap().digest(encoding),
            Algorithms::Md4 => self.md4.unwrap().digest(encoding),
            Algorithms::Md5 => self.md5.unwrap().digest(encoding),
            Algorithms::Sha1 => self.sha1.unwrap().digest(encoding),
            Algorithms::Sha224 => self.sha224.unwrap().digest(encoding),
            Algorithms::Sha256 => self.sha256.unwrap().digest(encoding),
            Algorithms::Sha384 => self.sha384.unwrap().digest(encoding),
            Algorithms::Sha512 => self.sha512.unwrap().digest(encoding),
            Algorithms::Sm3 => self.sm3.unwrap().digest(encoding),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md2() {
        let mut binding = Hash::create(Algorithms::Md2);
        let sut = binding.update(b"The quick brown fox jumps over the lazy dog");

        assert_eq!(&sut.digest(Hex), "03d85a0d629d2c442e987525319fc471");
        assert_eq!(
            &sut.digest(BinaryToTextEncoding::Base32),
            "APMFUDLCTUWEILUYOUSTDH6EOE"
        );
        assert_eq!(
            &sut.digest(BinaryToTextEncoding::Base64),
            "A9haDWKdLEQumHUlMZ/EcQ=="
        );
        assert_eq!(
            &sut.digest(BinaryToTextEncoding::Base64Url),
            "A9haDWKdLEQumHUlMZ_EcQ"
        );
        assert_eq!(&sut.digest(BinaryToTextEncoding::Binary), "00000011110110000101101000001101011000101001110100101100010001000010111010011000011101010010010100110001100111111100010001110001");
    }

    #[test]
    fn test_md4() {
        let sut = Hash::create(Algorithms::Md4)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "1bee69a46ba811185c194762abaeae90");
    }

    #[test]
    fn test_md5() {
        let sut = Hash::create(Algorithms::Md5)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_sha1() {
        let sut = Hash::create(Algorithms::Sha1)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn test_sha224() {
        let sut = Hash::create(Algorithms::Sha224)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);

        assert_eq!(
            sut,
            "730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525"
        );
    }

    #[test]
    fn test_sha256() {
        let sut = Hash::create(Algorithms::Sha256)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(
            sut,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }

    #[test]
    fn test_sha384() {
        let sut = Hash::create(Algorithms::Sha384)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "ca737f1014a48f4c0b6dd43cb177b0afd9e5169367544c494011e3317dbf9a509cb1e5dc1e85a941bbee3d7f2afbc9b1");
    }

    #[test]
    fn test_sha512() {
        let sut = Hash::create(Algorithms::Sha512)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6");
    }

    #[test]
    fn test_sm3() {
        let sut = Hash::create(Algorithms::Sm3)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(
            sut,
            "5fdfe814b8573ca021983970fc79b2218c9570369b4859684e2e4c3fc76cb8ea"
        );
    }
}
