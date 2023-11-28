use crypto::{digest::Digest, sha1::Sha1, sha2::Sha256};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HashStrategy {
    Sha1,
    Sha256,
}

impl HashStrategy {
    pub fn strategy_for(algorithm_name: &str) -> Option<HashStrategy> {
        match algorithm_name.to_lowercase().as_str() {
            "sha1" => Some(HashStrategy::Sha1),
            "sha256" => Some(HashStrategy::Sha256),
            _ => None,
        }
    }

    pub fn hasher_for(strategy: HashStrategy) -> Box<dyn Digest> {
        match strategy {
            HashStrategy::Sha1 => Box::new(Sha1::new()),
            HashStrategy::Sha256 => Box::new(Sha256::new()),
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

        assert_eq!(
            HashStrategy::strategy_for("sha256"),
            Some(HashStrategy::Sha256)
        );

        assert_eq!(HashStrategy::strategy_for("invalid"), None);
    }
}
