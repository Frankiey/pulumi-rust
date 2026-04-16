#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NvaNicType {
    /// The private NIC type
    PrivateNic,
    /// The public NIC type
    PublicNic,
    /// An additional private NIC type
    AdditionalPrivateNic,
    /// An additional public NIC type
    AdditionalPublicNic,
}

impl NvaNicType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PrivateNic => "PrivateNic",
            Self::PublicNic => "PublicNic",
            Self::AdditionalPrivateNic => "AdditionalPrivateNic",
            Self::AdditionalPublicNic => "AdditionalPublicNic",
        }
    }
}

impl From<NvaNicType> for serde_json::Value {
    fn from(v: NvaNicType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NvaNicType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NvaNicType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PrivateNic" => Ok(Self::PrivateNic),
            "PublicNic" => Ok(Self::PublicNic),
            "AdditionalPrivateNic" => Ok(Self::AdditionalPrivateNic),
            "AdditionalPublicNic" => Ok(Self::AdditionalPublicNic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["PrivateNic", "PublicNic", "AdditionalPrivateNic", "AdditionalPublicNic"])),
        }
    }
}
