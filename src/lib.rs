
// const {
//   ObjectDefineProperty,
//   ObjectDefineProperties,
// } = primordials;

// const {
//   assertCrypto,
//   deprecate,
// } = require('internal/util');
// assertCrypto();

// const {
//   ERR_CRYPTO_FIPS_FORCED,
//   ERR_WORKER_UNSUPPORTED_OPERATION,
// } = require('internal/errors').codes;
// const constants = internalBinding('constants').crypto;
// const { getOptionValue } = require('internal/options');
// const {
//   getFipsCrypto,
//   setFipsCrypto,
//   timingSafeEqual,
// } = internalBinding('crypto');
// const {
//   checkPrime,
//   checkPrimeSync,
//   generatePrime,
//   generatePrimeSync,
//   randomBytes,
//   randomFill,
//   randomFillSync,
//   randomInt,
//   randomUUID,
// } = require('internal/crypto/random');
// const {
//   pbkdf2,
//   pbkdf2Sync,
// } = require('internal/crypto/pbkdf2');
// const {
//   scrypt,
//   scryptSync,
// } = require('internal/crypto/scrypt');
// const {
//   hkdf,
//   hkdfSync,
// } = require('internal/crypto/hkdf');
// const {
//   generateKeyPair,
//   generateKeyPairSync,
//   generateKey,
//   generateKeySync,
// } = require('internal/crypto/keygen');
// const {
//   createSecretKey,
//   createPublicKey,
//   createPrivateKey,
//   KeyObject,
// } = require('internal/crypto/keys');
// const {
//   DiffieHellman,
//   DiffieHellmanGroup,
//   ECDH,
//   diffieHellman,
// } = require('internal/crypto/diffiehellman');
// const {
//   Cipher,
//   Cipheriv,
//   Decipher,
//   Decipheriv,
//   privateDecrypt,
//   privateEncrypt,
//   publicDecrypt,
//   publicEncrypt,
//   getCipherInfo,
// } = require('internal/crypto/cipher');
// const {
//   Sign,
//   signOneShot,
//   Verify,
//   verifyOneShot,
// } = require('internal/crypto/sig');
// const {
//   Hash,
//   Hmac,
// } = require('internal/crypto/hash');
// const {
//   X509Certificate,
// } = require('internal/crypto/x509');
// const {
//   getCiphers,
//   getCurves,
//   getHashes,
//   setEngine,
//   secureHeapUsed,
// } = require('internal/crypto/util');
// const Certificate = require('internal/crypto/certificate');

// let webcrypto;
// fn lazyWebCrypto() {
//   webcrypto ??= require('internal/crypto/webcrypto');
//   return webcrypto;
// }

// let ownsProcessState;
// fn lazyOwnsProcessState() {
//   ownsProcessState ??= require('internal/worker').ownsProcessState;
//   return ownsProcessState;
// } enum Ciphers { Aes128Cbc, Aes128CbcHmacSha1, Aes128CbcHmacSha256, Aes128Ccm, Aes128Cfb, Aes128Cfb1, Aes128Cfb8 Aes128Ctr Aes128Ecb Aes128Gcm Aes128Ocb Aes128Ofb Aes128Xts Aes192Cbc Aes192Ccm Aes192Cfb Aes192Cfb1 Aes192Cfb8 Aes192Ctr Aes192Ecb Aes192Gcm Aes192Ocb Aes192Ofb Aes256Cbc Aes256CbcHmacSha1 Aes256CbcHmacSha256 Aes256Ccm Aes256Cfb Aes256Cfb1 Aes256Cfb8 Aes256Ctr Aes256Ecb Aes256Gcm Aes256Ocb Aes256Ofb Aes256Xts Aes128 Aes128Wrap Aes192 Aes192Wrap Aes256 Aes256Wrap Bf BfCbc BfCfb BfEcb BfOfb Blowfish Camellia128Cbc Camellia128Cfb Camellia128Cfb1 Camellia128Cfb8 Camellia128Ecb Camellia128Ofb Camellia192Cbc Camellia192Cfb Camellia192Cfb1 Camellia192Cfb8 }

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

// class Hash extends stream.Transform {
//   private constructor();
//   /**
//    * Creates a new `Hash` object that contains a deep copy of the internal state
//    * of the current `Hash` object.
//    *
//    * The optional `options` argument controls stream behavior. For XOF hash
//    * functions such as `'shake256'`, the `outputLength` option can be used to
//    * specify the desired output length in bytes.
//    *
//    * An error is thrown when an attempt is made to copy the `Hash` object after
//    * its `hash.digest()` method has been called.
//    *
//    * ```js
//    * // Calculate a rolling hash.
//    * const {
//    *   createHash,
//    * } = await import('node:crypto');
//    *
//    * const hash = createHash('sha256');
//    *
//    * hash.update('one');
//    * console.log(hash.copy().digest('hex'));
//    *
//    * hash.update('two');
//    * console.log(hash.copy().digest('hex'));
//    *
//    * hash.update('three');
//    * console.log(hash.copy().digest('hex'));
//    *
//    * // Etc.
//    * ```
//    * @since v13.1.0
//    * @param options `stream.transform` options
//    */
//   copy(options?: stream.TransformOptions): Hash;
//   /**
//    * Updates the hash content with the given `data`, the encoding of which
//    * is given in `inputEncoding`.
//    * If `encoding` is not provided, and the `data` is a string, an
//    * encoding of `'utf8'` is enforced. If `data` is a `Buffer`, `TypedArray`, or`DataView`, then `inputEncoding` is ignored.
//    *
//    * This can be called many times with new data as it is streamed.
//    * @since v0.1.92
//    * @param inputEncoding The `encoding` of the `data` string.
//    */
//   update(data: BinaryLike): Hash;
//   update(data: string, inputEncoding: Encoding): Hash;
//   /**
//    * Calculates the digest of all of the data passed to be hashed (using the `hash.update()` method).
//    * If `encoding` is provided a string will be returned; otherwise
//    * a `Buffer` is returned.
//    *
//    * The `Hash` object can not be used again after `hash.digest()` method has been
//    * called. Multiple calls will cause an error to be thrown.
//    * @since v0.1.92
//    * @param encoding The `encoding` of the return value.
//    */
//   digest(): Buffer;
//   digest(encoding: BinaryToTextEncoding): string;
// }
mod algorithms;
mod utils;
mod sig;

#[derive(Copy, Clone)]
pub enum BinaryToTextEncoding {
    Base32,
    Base64,
    Base64Url,
    Hex,
    Binary,
}

#[allow(dead_code)]
pub enum CharacterEncoding {
    Utf8,
    Utf16le,
    Latin1,
}

pub enum LegacyCharacterEncoding {
    Ascii,
    Binary,
    Ucs2,
}
pub enum Encoding {
    BinaryToTextEncoding(BinaryToTextEncoding),
    CharacterEncoding(CharacterEncoding),
    LegacyCharacterEncoding(LegacyCharacterEncoding),
}
pub enum ECDHKeyFormat {
    Compressed,
    Uncompressed,
    Hybrid,
}

#[derive(Copy, Clone)]
pub enum Algorithms {
    // GostMac,
    Md2, // ok
    Md4, // ok
    Md5, // ok
    // MdGost94,
    // Ripemd160,
    Sha1, // ok
    Sha224, // ok
    Sha256, // ok
    Sha384, // ok
    Sha512, // ok
    Sm3, // ok
    // Sha3_224,
    // Sha3_256,
    // Sha3_384,
    // Sha3_512,
    // Streebog256,
    // Streebog512,
    // Whirlpool,
}

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

    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        match self.algorithm {
            Algorithms::Md2 => {
                if let Some(md2) = self.md2.as_mut() {
                    md2.update(data);
                }
            },
            Algorithms::Md4 => {
                if let Some(md4) = self.md4.as_mut() {
                    md4.update(data);
                }
            },
            Algorithms::Md5 => {
                if let Some(md5) = self.md5.as_mut() {
                    md5.update(data);
                }
            },
            Algorithms::Sha1 => {
                if let Some(sha1) = self.sha1.as_mut() {
                    sha1.update(data);
                }
            },
            Algorithms::Sha224 => {
                if let Some(sha224) = self.sha224.as_mut() {
                    sha224.update(data);
                }
            },
            Algorithms::Sha256 => {
                if let Some(sha256) = self.sha256.as_mut() {
                    sha256.update(data);
                }
                // else {
                //     let mut sha256 = Sha256::new();
                //     sha256.update(data);
                //     self.sha256 = Some(sha256);
                // }
            },
            Algorithms::Sha384 => {
                if let Some(sha384) = self.sha384.as_mut() {
                    sha384.update(data);
                }
            },
            Algorithms::Sha512 => {
                if let Some(sha512) = self.sha512.as_mut() {
                    sha512.update(data);
                }
            },
            Algorithms::Sm3 => {
                if let Some(sm3) = self.sm3.as_mut() {
                    sm3.update(data);
                }
            },
        }
        self
    }

    pub fn digest(&mut self, encoding: BinaryToTextEncoding) -> String{
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


// trait CrabCryptTrait {
//   fn create_hash(algorithm: Algorithms) -> (); // (algorithm, options) // new Hash(algorithm, options);
//   fn create_cipher() -> (); // (cipher, password, options) // new Cipher(cipher, password, options);
//   fn create_cipheriv() -> (); // (cipher, key, iv, options) // new Cipheriv(cipher, key, iv, options)
//   fn create_decipher() -> (); // (cipher, password, options) // new Decipher(cipher, password, options)
//   fn create_decipheriv() -> (); // (cipher, key, iv, options) // new Decipheriv(cipher, key, iv, options)
//   fn create_diffie_hellman() -> (); // (sizeOrKey, keyEncoding, generator, genEncoding) // new DiffieHellman(sizeOrKey, keyEncoding, generator, genEncoding)
//   fn create_diffie_hellman_group() -> (); // (name) // new DiffieHellmanGroup(name)
//   fn create_ecdh() -> (); // (curve) // new ECDH(curve)
//   fn create_hmac() -> (); // (hmac, key, options) // new Hmac(hmac, key, options)
//   fn create_sign() -> (); // (algorithm, options) // new Sign(algorithm, options)
//   fn create_verify() -> (); // (algorithm, options) // new Verify(algorithm, options);
// }


// These helper functions are needed because the constructors can
// use new, in which case V8 cannot inline the recursive constructor call
// fn createHash(algorithm, options) {
//   return new Hash(algorithm, options);
// }

// fn createCipher(cipher, password, options) {
//   return new Cipher(cipher, password, options);
// }

// fn createCipheriv(cipher, key, iv, options) {
//   return new Cipheriv(cipher, key, iv, options);
// }

// fn createDecipher(cipher, password, options) {
//   return new Decipher(cipher, password, options);
// }

// fn createDecipheriv(cipher, key, iv, options) {
//   return new Decipheriv(cipher, key, iv, options);
// }

// fn createDiffieHellman(sizeOrKey, keyEncoding, generator, genEncoding) {
//   return new DiffieHellman(sizeOrKey, keyEncoding, generator, genEncoding);
// }

// fn createDiffieHellmanGroup(name) {
//   return new DiffieHellmanGroup(name);
// }

// fn createECDH(curve) {
//   return new ECDH(curve);
// }

// fn createHmac(hmac, key, options) {
//   return new Hmac(hmac, key, options);
// }

// fn createSign(algorithm, options) {
//   return new Sign(algorithm, options);
// }

// fn createVerify(algorithm, options) {
//   return new Verify(algorithm, options);
// }

// module.exports = {
//   // Methods

//   checkPrime,
//   checkPrimeSync,

//   createCipheriv,
//   createDecipheriv,
//   createDiffieHellman,
//   createDiffieHellmanGroup,
//   createECDH,
//   createHash,
//   createHmac,
//   createPrivateKey,
//   createPublicKey,
//   createSecretKey,
//   createSign,
//   createVerify,

//   diffieHellman,

//   generatePrime,
//   generatePrimeSync,

//   getCiphers,
//   getCipherInfo,
//   getCurves,
//   getDiffieHellman: createDiffieHellmanGroup,
//   getHashes,

//   hkdf,
//   hkdfSync,
//   pbkdf2,
//   pbkdf2Sync,

//   generateKeyPair,
//   generateKeyPairSync,
//   generateKey,
//   generateKeySync,

//   privateDecrypt,
//   privateEncrypt,

//   publicDecrypt,
//   publicEncrypt,

//   randomBytes,
//   randomFill,
//   randomFillSync,
//   randomInt,
//   randomUUID,

//   scrypt,
//   scryptSync,
//   sign: signOneShot,
//   setEngine,
//   timingSafeEqual,
//   getFips,
//   setFips,
//   verify: verifyOneShot,

//   // Classes
//   Certificate,
//   Cipher,
//   Cipheriv,
//   Decipher,
//   Decipheriv,
//   DiffieHellman,
//   DiffieHellmanGroup,
//   ECDH,
//   Hash,
//   Hmac,
//   KeyObject,
//   Sign,
//   Verify,
//   X509Certificate,
//   secureHeapUsed,
// };

// fn getFips() {
//   return getOptionValue('--force-fips') ? 1 : getFipsCrypto();
// }

// fn setFips(val) {
//   if (getOptionValue('--force-fips')) {
//     if (val) return;
//     throw new ERR_CRYPTO_FIPS_FORCED();
//   } else {
//     if (!lazyOwnsProcessState()) {
//       throw new ERR_WORKER_UNSUPPORTED_OPERATION('Calling crypto.setFips()');
//     }
//     setFipsCrypto(val);
//   }
// }

// fn getRandomValues(array) {
//   return lazyWebCrypto().crypto.getRandomValues(array);
// }

// ObjectDefineProperty(constants, 'defaultCipherList', {
//   __proto__: null,
//   get() {
//     const value = getOptionValue('--tls-cipher-list');
//     ObjectDefineProperty(this, 'defaultCipherList', {
//       __proto__: null,
//       writable: true,
//       configurable: true,
//       enumerable: true,
//       value,
//     });
//     return value;
//   },
//   set(val) {
//     ObjectDefineProperty(this, 'defaultCipherList', {
//       __proto__: null,
//       writable: true,
//       configurable: true,
//       enumerable: true,
//       value: val,
//     });
//   },
//   configurable: true,
//   enumerable: true,
// });

// fn getRandomBytesAlias(key) {
//   return {
//     enumerable: false,
//     configurable: true,
//     get() {
//       let value;
//       if (getOptionValue('--pending-deprecation')) {
//         value = deprecate(
//           randomBytes,
//           `crypto.${key} is deprecated.`,
//           'DEP0115');
//       } else {
//         value = randomBytes;
//       }
//       ObjectDefineProperty(
//         this,
//         key,
//         {
//           __proto__: null,
//           enumerable: false,
//           configurable: true,
//           writable: true,
//           value: value,
//         },
//       );
//       return value;
//     },
//     set(value) {
//       ObjectDefineProperty(
//         this,
//         key,
//         {
//           __proto__: null,
//           enumerable: true,
//           configurable: true,
//           writable: true,
//           value,
//         },
//       );
//     },
//   };
// }

// ObjectDefineProperties(module.exports, {
//   createCipher: {
//     __proto__: null,
//     enumerable: false,
//     value: deprecate(createCipher,
//                      'crypto.createCipher is deprecated.', 'DEP0106'),
//   },
//   createDecipher: {
//     __proto__: null,
//     enumerable: false,
//     value: deprecate(createDecipher,
//                      'crypto.createDecipher is deprecated.', 'DEP0106'),
//   },
//   // crypto.fips is deprecated. DEP0093. Use crypto.getFips()/crypto.setFips()
//   fips: {
//     __proto__: null,
//     get: getFips,
//     set: setFips,
//   },
//   constants: {
//     __proto__: null,
//     configurable: false,
//     enumerable: true,
//     value: constants,
//   },

//   webcrypto: {
//     __proto__: null,
//     configurable: false,
//     enumerable: true,
//     get() { return lazyWebCrypto().crypto; },
//     set: undefined,
//   },

//   subtle: {
//     __proto__: null,
//     configurable: false,
//     enumerable: true,
//     get() { return lazyWebCrypto().crypto.subtle; },
//     set: undefined,
//   },

//   getRandomValues: {
//     __proto__: null,
//     configurable: false,
//     enumerable: true,
//     get: () => getRandomValues,
//     set: undefined,
//   },

//   // Aliases for randomBytes are deprecated.
//   // The ecosystem needs those to exist for backwards compatibility.
//   prng: getRandomBytesAlias('prng'),
//   pseudoRandomBytes: getRandomBytesAlias('pseudoRandomBytes'),
//   rng: getRandomBytesAlias('rng'),
// });

























#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md2() {
        let mut binding = Hash::create(Algorithms::Md2);
        let sut = binding
            .update(b"The quick brown fox jumps over the lazy dog");

        assert_eq!(&sut.digest(Hex), "03d85a0d629d2c442e987525319fc471");
        assert_eq!(&sut.digest(BinaryToTextEncoding::Base32), "APMFUDLCTUWEILUYOUSTDH6EOE");
        assert_eq!(&sut.digest(BinaryToTextEncoding::Base64), "A9haDWKdLEQumHUlMZ/EcQ==");
        assert_eq!(&sut.digest(BinaryToTextEncoding::Base64Url), "A9haDWKdLEQumHUlMZ_EcQ");
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

        assert_eq!(sut, "730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525");
    }

    #[test]
    fn test_sha256() {
        let sut = Hash::create(Algorithms::Sha256)
            .update(b"The quick brown fox jumps over the lazy dog")
            .digest(Hex);
        assert_eq!(sut, "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592");
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
        assert_eq!(sut, "5fdfe814b8573ca021983970fc79b2218c9570369b4859684e2e4c3fc76cb8ea");
    }
}