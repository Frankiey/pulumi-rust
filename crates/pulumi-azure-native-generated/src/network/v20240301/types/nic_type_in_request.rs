/// NIC type. This should be either PublicNic or PrivateNic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NicTypeInRequest {
    PublicNic,
    PrivateNic,
}

impl NicTypeInRequest {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PublicNic => "PublicNic",
            Self::PrivateNic => "PrivateNic",
        }
    }
}

impl From<NicTypeInRequest> for serde_json::Value {
    fn from(v: NicTypeInRequest) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NicTypeInRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NicTypeInRequest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PublicNic" => Ok(Self::PublicNic),
            "PrivateNic" => Ok(Self::PrivateNic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["PublicNic", "PrivateNic"])),
        }
    }
}
