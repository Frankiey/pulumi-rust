/// The protocol for which the DDoS protection policy is being customized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdosCustomPolicyProtocol {
    Tcp,
    Udp,
    Syn,
}

impl DdosCustomPolicyProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "Tcp",
            Self::Udp => "Udp",
            Self::Syn => "Syn",
        }
    }
}

impl From<DdosCustomPolicyProtocol> for serde_json::Value {
    fn from(v: DdosCustomPolicyProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DdosCustomPolicyProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DdosCustomPolicyProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tcp" => Ok(Self::Tcp),
            "Udp" => Ok(Self::Udp),
            "Syn" => Ok(Self::Syn),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tcp", "Udp", "Syn"])),
        }
    }
}
