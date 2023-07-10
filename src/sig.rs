// use std::convert::TryInto;
// use std::io::{self, Write};
// use std::ops::Deref;
// use std::os::raw::{c_char, c_int};
// use std::ptr::{null, null_mut};
// use std::slice;

// use openssl::error::ErrorStack;
// use openssl::hash::{Hasher, MessageDigest};
// use openssl::pkey::{PKey, Private, Public};
// use openssl::sign::{Signer, Verifier};
// use openssl::symm::{decrypt, encrypt, Cipher};
// use openssl::x509::X509;

// use crate::error::{Error, Result};

// pub struct Sign {
//     signer: Signer<'static>,
// }

// impl Sign {
//     pub fn new(algorithm: MessageDigest) -> Result<Self> {
//         let pkey = PKey::generate(2048)?;
//         let signer = Signer::new(algorithm, &pkey)?;
//         Ok(Self { signer })
//     }

//     pub fn update(&mut self, data: &[u8]) -> Result<()> {
//         self.signer.update(data)?;
//         Ok(())
//     }

//     pub fn sign(&mut self) -> Result<Vec<u8>> {
//         let signature = self.signer.sign_to_vec()?;
//         Ok(signature)
//     }
// }

// pub struct Verify {
//     verifier: Verifier<'static>,
// }

// impl Verify {
//     pub fn new(algorithm: MessageDigest, public_key: &X509) -> Result<Self> {
//         let public_key = public_key.public_key()?;
//         let verifier = Verifier::new(algorithm, &public_key)?;
//         Ok(Self { verifier })
//     }

//     pub fn update(&mut self, data: &[u8]) -> Result<()> {
//         self.verifier.update(data)?;
//         Ok(())
//     }

//     pub fn verify(&mut self, signature: &[u8]) -> Result<bool> {
//         let result = self.verifier.verify(signature)?;
//         Ok(result)
//     }
// }

// pub fn encrypt_aes_256_cbc(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>> {
//     let cipher = Cipher::aes_256_cbc();
//     let mut ciphertext = vec![0; data.len() + cipher.block_size()];
//     let len = encrypt(cipher, key, Some(iv), data, &mut ciphertext)?;
//     ciphertext.truncate(len);
//     Ok(ciphertext)
// }

// pub fn decrypt_aes_256_cbc(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
//     let cipher = Cipher::aes_256_cbc();
//     let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
//     let len = decrypt(cipher, key, Some(iv), ciphertext, &mut plaintext)?;
//     plaintext.truncate(len);
//     Ok(plaintext)
// }

// pub fn sha256(data: &[u8]) -> Result<Vec<u8>> {
//     let mut hasher = Hasher::new(MessageDigest::sha256())?;
//     hasher.update(data)?;
//     let digest = hasher.finish()?;
//     Ok(digest.to_vec())
// }

// pub fn sha512(data: &[u8]) -> Result<Vec<u8>> {
//     let mut hasher = Hasher::new(MessageDigest::sha512())?;
//     hasher.update(data)?;
//     let digest = hasher.finish()?;
//     Ok(digest.to_vec())
// }

// pub fn base64_encode(data: &[u8]) -> String {
//     base64::encode(data)
// }

// pub fn base64_decode(data: &str) -> Result<Vec<u8>> {
//     let decoded = base64::decode(data)?;
//     Ok(decoded)
// }

// pub fn hex_encode(data: &[u8]) -> String {
//     hex::encode(data)
// }

// pub fn hex_decode(data: &str) -> Result<Vec<u8>> {
//     let decoded = hex::decode(data)?;
//     Ok(decoded)
// }

// pub fn rsa_sign(algorithm: MessageDigest, private_key: &PKey<Private>, data: &[u8]) -> Result<Vec<u8>> {
//     let mut signer = Signer::new(algorithm, private_key)?;
//     signer.update(data)?;
//     let signature = signer.sign_to_vec()?;
//     Ok(signature)
// }

// pub fn rsa_verify(algorithm: MessageDigest, public_key: &PKey<Public>, signature: &[u8], data: &[u8]) -> Result<bool> {
//     let mut verifier = Verifier::new(algorithm, public_key)?;
//     verifier.update(data)?;
//     let result = verifier.verify(signature)?;
//     Ok(result)
// }

// pub fn rsa_encrypt(public_key: &PKey<Public>, data: &[u8]) -> Result<Vec<u8>> {
//     let mut ciphertext = vec![0; public_key.size() as usize];
//     let len = public_key.encrypt(data, &mut ciphertext, openssl::rsa::Padding::PKCS1)?;
//     ciphertext.truncate(len);
//     Ok(ciphertext)
// }

// pub fn rsa_decrypt(private_key: &PKey<Private>, ciphertext: &[u8]) -> Result<Vec<u8>> {
//     let mut plaintext = vec![0; private_key.size() as usize];
//     let len = private_key.decrypt(ciphertext, &mut plaintext, openssl::rsa::Padding::PKCS1)?;
//     plaintext.truncate(len);
//     Ok(plaintext)
// }

// pub fn aes_256_gcm_encrypt(key: &[u8], iv: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
//     let cipher = Cipher::aes_256_gcm();
//     let mut ciphertext = vec![0; data.len() + cipher.block_size()];
//     let mut tag = vec![0; cipher.block_size()];
//     let len = encrypt(cipher, key, Some(iv), aad, data, &mut ciphertext, &mut tag)?;
//     ciphertext.truncate(len);
//     ciphertext.extend_from_slice(&tag);
//     Ok(ciphertext)
// }

// pub fn aes_256_gcm_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
//     let cipher = Cipher::aes_256_gcm();
//     let tag_len = cipher.block_size();
//     let data_len = ciphertext.len() - tag_len;
//     let (data, tag) = ciphertext.split_at(data_len);
//     let mut plaintext = vec![0; data_len + cipher.block_size()];
//     let len = decrypt(cipher, key, Some(iv), aad, data, tag, &mut plaintext)?;
//     plaintext.truncate(len);
//     Ok(plaintext)
// }