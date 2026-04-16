/// Address prefix type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressPrefixType {
    IPPrefix,
    ServiceTag,
}

impl AddressPrefixType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPPrefix => "IPPrefix",
            Self::ServiceTag => "ServiceTag",
        }
    }
}

impl From<AddressPrefixType> for serde_json::Value {
    fn from(v: AddressPrefixType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AddressPrefixType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AddressPrefixType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPPrefix" => Ok(Self::IPPrefix),
            "ServiceTag" => Ok(Self::ServiceTag),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPPrefix", "ServiceTag"])),
        }
    }
}
