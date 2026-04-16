/// The type of this virtual network gateway. Possible values are: 'PolicyBased' and 'RouteBased'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnType {
    PolicyBased,
    RouteBased,
}

impl VpnType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PolicyBased => "PolicyBased",
            Self::RouteBased => "RouteBased",
        }
    }
}

impl From<VpnType> for serde_json::Value {
    fn from(v: VpnType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PolicyBased" => Ok(Self::PolicyBased),
            "RouteBased" => Ok(Self::RouteBased),
            _ => Err(serde::de::Error::unknown_variant(&s, &["PolicyBased", "RouteBased"])),
        }
    }
}
