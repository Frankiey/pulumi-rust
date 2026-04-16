/// The IKE integrity algorithm (IKE phase 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IkeIntegrity {
    MD5,
    SHA1,
    SHA256,
    SHA384,
    GCMAES256,
    GCMAES128,
}

impl IkeIntegrity {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MD5 => "MD5",
            Self::SHA1 => "SHA1",
            Self::SHA256 => "SHA256",
            Self::SHA384 => "SHA384",
            Self::GCMAES256 => "GCMAES256",
            Self::GCMAES128 => "GCMAES128",
        }
    }
}

impl From<IkeIntegrity> for serde_json::Value {
    fn from(v: IkeIntegrity) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IkeIntegrity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IkeIntegrity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "MD5" => Ok(Self::MD5),
            "SHA1" => Ok(Self::SHA1),
            "SHA256" => Ok(Self::SHA256),
            "SHA384" => Ok(Self::SHA384),
            "GCMAES256" => Ok(Self::GCMAES256),
            "GCMAES128" => Ok(Self::GCMAES128),
            _ => Err(serde::de::Error::unknown_variant(&s, &["MD5", "SHA1", "SHA256", "SHA384", "GCMAES256", "GCMAES128"])),
        }
    }
}
