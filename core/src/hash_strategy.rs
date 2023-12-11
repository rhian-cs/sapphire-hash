use strum::Display;
use strum::EnumIter;
use strum::EnumString;

use openssl::error::ErrorStack as OpenSSLErrorStack;
use openssl::hash::Hasher as OpenSSLHasher;
use openssl::hash::MessageDigest;

#[derive(Clone, Copy, PartialEq, Debug, EnumString, EnumIter, Display)]
#[strum(serialize_all = "lowercase")]
pub enum HashStrategy {
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
    Ripemd160,
    Sm3,
}

impl HashStrategy {
    pub fn hasher(&self) -> Result<OpenSSLHasher, OpenSSLErrorStack> {
        let message_digest = match self {
            HashStrategy::Md5 => MessageDigest::md5(),
            HashStrategy::Sha1 => MessageDigest::sha1(),
            HashStrategy::Sha224 => MessageDigest::sha224(),
            HashStrategy::Sha256 => MessageDigest::sha256(),
            HashStrategy::Sha384 => MessageDigest::sha384(),
            HashStrategy::Sha512 => MessageDigest::sha512(),
            HashStrategy::Sha3_224 => MessageDigest::sha3_224(),
            HashStrategy::Sha3_256 => MessageDigest::sha3_256(),
            HashStrategy::Sha3_384 => MessageDigest::sha3_384(),
            HashStrategy::Sha3_512 => MessageDigest::sha3_512(),
            HashStrategy::Shake128 => MessageDigest::shake_128(),
            HashStrategy::Shake256 => MessageDigest::shake_256(),
            HashStrategy::Ripemd160 => MessageDigest::ripemd160(),
            HashStrategy::Sm3 => MessageDigest::sm3(),
        };

        OpenSSLHasher::new(message_digest)
    }
}
