use openssl::error::ErrorStack as OpenSSLErrorStack;
use openssl::hash::Hasher as OpenSSLHasher;
use openssl::hash::MessageDigest;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub fn strategy_for(algorithm_name: &str) -> Option<HashStrategy> {
        match algorithm_name.to_lowercase().as_str() {
            "md5" => Some(HashStrategy::Md5),
            "sha1" => Some(HashStrategy::Sha1),
            "sha224" => Some(HashStrategy::Sha224),
            "sha256" => Some(HashStrategy::Sha256),
            "sha384" => Some(HashStrategy::Sha384),
            "sha512" => Some(HashStrategy::Sha512),
            "sha3_224" => Some(HashStrategy::Sha3_224),
            "sha3_256" => Some(HashStrategy::Sha3_256),
            "sha3_384" => Some(HashStrategy::Sha3_384),
            "sha3_512" => Some(HashStrategy::Sha3_512),
            "shake128" => Some(HashStrategy::Shake128),
            "shake256" => Some(HashStrategy::Shake256),
            "ripemd160" => Some(HashStrategy::Ripemd160),
            "sm3" => Some(HashStrategy::Sm3),
            _ => None,
        }
    }

    pub fn hasher_for(strategy: HashStrategy) -> Result<OpenSSLHasher, OpenSSLErrorStack> {
        match strategy {
            HashStrategy::Md5 => OpenSSLHasher::new(MessageDigest::md5()),
            HashStrategy::Sha1 => OpenSSLHasher::new(MessageDigest::sha1()),
            HashStrategy::Sha224 => OpenSSLHasher::new(MessageDigest::sha224()),
            HashStrategy::Sha256 => OpenSSLHasher::new(MessageDigest::sha256()),
            HashStrategy::Sha384 => OpenSSLHasher::new(MessageDigest::sha384()),
            HashStrategy::Sha512 => OpenSSLHasher::new(MessageDigest::sha512()),
            HashStrategy::Sha3_224 => OpenSSLHasher::new(MessageDigest::sha3_224()),
            HashStrategy::Sha3_256 => OpenSSLHasher::new(MessageDigest::sha3_256()),
            HashStrategy::Sha3_384 => OpenSSLHasher::new(MessageDigest::sha3_384()),
            HashStrategy::Sha3_512 => OpenSSLHasher::new(MessageDigest::sha3_512()),
            HashStrategy::Shake128 => OpenSSLHasher::new(MessageDigest::shake_128()),
            HashStrategy::Shake256 => OpenSSLHasher::new(MessageDigest::shake_256()),
            HashStrategy::Ripemd160 => OpenSSLHasher::new(MessageDigest::ripemd160()),
            HashStrategy::Sm3 => OpenSSLHasher::new(MessageDigest::sm3()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash_strategy::HashStrategy;

    #[test]
    fn test_strategy_for() {
        assert_eq!(HashStrategy::strategy_for("sha1"), Some(HashStrategy::Sha1));
        assert_eq!(HashStrategy::strategy_for("SHA1"), Some(HashStrategy::Sha1));
        assert_eq!(HashStrategy::strategy_for("Sha1"), Some(HashStrategy::Sha1));

        assert_eq!(HashStrategy::strategy_for("sha256"), Some(HashStrategy::Sha256));

        assert_eq!(HashStrategy::strategy_for("invalid"), None);
    }
}
