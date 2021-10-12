#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BIP32Path(pub slip10::BIP32Path);
