use crypto::{digest::Digest, sha1::Sha1};

#[derive(Clone, Copy)]
pub enum HashStrategy {
    Sha1,
}

impl HashStrategy {
    pub fn hasher_for(strategy: HashStrategy) -> Box<dyn Digest> {
        match strategy {
            HashStrategy::Sha1 => Box::new(Sha1::new()),
        }
    }
}
