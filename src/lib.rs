
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

// enum Algorithms {
//   Ciphers(Ciphers),
// }

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
mod crabcrypt {
    pub enum BinaryToTextEncoding {
        Base64,
        Base64Url,
        Hex,
        Binary,
    }
    // type CharacterEncoding = 'utf8' | 'utf-8' | 'utf16le' | 'latin1';
    // type LegacyCharacterEncoding = 'ascii' | 'binary' | 'ucs2' | 'ucs-2';
    // type Encoding = BinaryToTextEncoding | CharacterEncoding | LegacyCharacterEncoding;
    // type ECDHKeyFormat = 'compressed' | 'uncompressed' | 'hybrid';
    pub enum Algorithms {
        GostMac,
        Md4,
        Md5,
        MdGost94,
        Ripemd160,
        Sha1,
        Sha224,
        Sha256,
        Sha384,
        Sha512,
        Streebog256,
        Streebog512,
        Whirlpool,
    }
    pub struct Hash {
        pub algorithm: Algorithms,
        // GostMac,
        // Md4,
        // Md5,
        // MdGost94,
        // Ripemd160,
        // Sha1,
        // Sha224,
        pub sha256: Option<crate::algorithms::sha256::Sha256>,
        // Sha384,
        // Sha512,
        // Streebog256,
        // Streebog512,
        // Whirlpool,
    }
    impl Hash {
        pub fn create(algorithm: Algorithms) -> Hash {
            // Implementação da função create() do módulo Hash
            println!("Creating a hash");
            let sha256: Option<crate::algorithms::sha256::Sha256>;
            match algorithm {
                Algorithms::Sha256 => {
                    sha256 = Some(crate::algorithms::sha256::Sha256::new());
                }
                _ => {
                    sha256 = None;
                }
            }
            Hash {
                algorithm,
                sha256,
            }
        }
pub fn update(&mut self, data: &[u8]) -> &mut Self {
    match self.algorithm {
        Algorithms::Sha256 => {
            if let Some(sha256) = self.sha256.as_mut() {
                sha256.update(data);
            } else {
                let mut sha256 = crate::algorithms::sha256::Sha256::new();
                sha256.update(data);
                self.sha256 = Some(sha256);
            }
        }
        _ => {}
    }
    self
}

        pub fn digest(&self, encoding: BinaryToTextEncoding) {
            // Implementação da função digest() do módulo Hash
            println!("Digesting a hash");
        }

        pub fn end(&self) {
            // Implementação da função end() do módulo Hash
            println!("Ending a hash");
        }
    }
}

use crabcrypt::{Algorithms::Sha256, BinaryToTextEncoding::Hex};
fn a() {
    let crabcrypt = crabcrypt::Hash::create(Sha256)
        .update("Hello World".as_bytes())
        .digest(Hex);
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



























// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
