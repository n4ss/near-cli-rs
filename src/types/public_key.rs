#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct PublicKey(pub near_crypto::PublicKey);

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for PublicKey {
    type Err = near_crypto::ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let public_key = near_crypto::PublicKey::from_str(s)?;
        Ok(Self(public_key))
    }
}
