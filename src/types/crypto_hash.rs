use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct CryptoHash(pub near_primitives::hash::CryptoHash);

impl std::fmt::Display for CryptoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CryptoHash {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crypto_hash = near_primitives::hash::CryptoHash::from_str(s)?;
        Ok(Self(crypto_hash))
    }
}

impl From<std::option::Option<CryptoHash>> for CryptoHash {
    fn from(option: std::option::Option<CryptoHash>) -> Self {
        match option {
            Some(crypto_hash) => crypto_hash,
            None => Self::default(),
        }
    }
}
