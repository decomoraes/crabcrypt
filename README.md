<p style="text-align: center;">
  <img src="./crabcrypt.webp" style="width: 200px; height: 200px;" />
</p>

---

# crabcrypt

![Crabcrypt Status](https://img.shields.io/badge/Status-In%20Development-orange)
![Implemented Features](https://img.shields.io/badge/Implemented-Hashing-green)


Welcome to Crabcrypt, a comprehensive cryptography library currently being developed in Rust.

## Status

The project is currently **under development**. As of now, the hashing functionality has been implemented.

## Features

Here are some of the main features that we plan to provide:

- A range of secure hashing algorithms including MD2, MD4, MD5, SHA1, SHA224, SHA256, SHA384, SHA512, and SM3.
- Support for multiple character encodings.
- Cipher options for advanced encryption and decryption methods (under development).
- Efficient and convenient APIs for developers.

## Implemented

### Hashing

Crabcrypt provides a set of hashing algorithms including:

- MD2
- MD4
- MD5
- SHA1
- SHA224
- SHA256
- SHA384
- SHA512
- SM3

More features will be implemented soon. Stay tuned!

## Usage

Here's an example of how to use the hashing feature in Crabcrypt:

```rust
use crabcrypt::{
    Algorithms,
    BinaryToTextEncoding::Hex,
    Hash
};

fn main() {
    let hash_md5 = Hash::create(Algorithms::Md5)
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(Hex);
    println!("MD5: {}", hash_md5);

    let hash_sha1 = Hash::create(Algorithms::Sha1)
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(Hex);
    println!("SHA-1: {}", hash_sha1);

    let hash_sha256 = Hash::create(Algorithms::Sha256)
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(Hex);
    println!("SHA-256: {}", hash_sha256);

    let hash_sha512 = Hash::create(Algorithms::Sha512)
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(Hex);
    println!("SHA-512: {}", hash_sha512);

    let hash_sm3 = Hash::create(Algorithms::Sm3)
        .update(b"The quick brown fox jumps over the lazy dog")
        .digest(Hex);
    println!("SM3: {}", hash_sm3);

    // Create a mutable hash for the SHA256 algorithm
    let mut hash_sha256_2 = Hash::create(Algorithms::Sha256);

    // First update
    hash_sha256_2.update(b"The quick brown fox ");

    // Second update
    hash_sha256_2.update(b"jumps over the lazy dog");

    // Elsewhere in the code, the digest is calculated
    let sha256_result_2 = hash_sha256_2.digest(Hex);
    println!("SHA-256 (updated twice): {}", sha256_result_2);
}
```

## Contribution

As Crabcrypt is in its early development stages, we appreciate any and all contributions! Whether it's reporting bugs, suggesting new features, improving documentation, or writing code, all contributions are welcome.

## License

Crabcrypt is [MIT licensed](LICENSE).