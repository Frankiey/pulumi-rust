/// The IKE encryption algorithm (IKE phase 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IkeEncryption {
    DES,
    DES3,
    AES128,
    AES192,
    AES256,
    GCMAES256,
    GCMAES128,
}

impl IkeEncryption {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DES => "DES",
            Self::DES3 => "DES3",
            Self::AES128 => "AES128",
            Self::AES192 => "AES192",
            Self::AES256 => "AES256",
            Self::GCMAES256 => "GCMAES256",
            Self::GCMAES128 => "GCMAES128",
        }
    }
}

impl From<IkeEncryption> for serde_json::Value {
    fn from(v: IkeEncryption) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IkeEncryption {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IkeEncryption {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DES" => Ok(Self::DES),
            "DES3" => Ok(Self::DES3),
            "AES128" => Ok(Self::AES128),
            "AES192" => Ok(Self::AES192),
            "AES256" => Ok(Self::AES256),
            "GCMAES256" => Ok(Self::GCMAES256),
            "GCMAES128" => Ok(Self::GCMAES128),
            _ => Err(serde::de::Error::unknown_variant(&s, &["DES", "DES3", "AES128", "AES192", "AES256", "GCMAES256", "GCMAES128"])),
        }
    }
}
