/// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedCopyScope {
    PrivateLink,
    AAD,
}

impl AllowedCopyScope {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PrivateLink => "PrivateLink",
            Self::AAD => "AAD",
        }
    }
}

impl From<AllowedCopyScope> for serde_json::Value {
    fn from(v: AllowedCopyScope) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AllowedCopyScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AllowedCopyScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PrivateLink" => Ok(Self::PrivateLink),
            "AAD" => Ok(Self::AAD),
            _ => Err(serde::de::Error::unknown_variant(&s, &["PrivateLink", "AAD"])),
        }
    }
}
