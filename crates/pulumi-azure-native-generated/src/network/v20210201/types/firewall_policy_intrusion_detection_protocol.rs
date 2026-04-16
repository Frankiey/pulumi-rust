/// The rule bypass protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyIntrusionDetectionProtocol {
    TCP,
    UDP,
    ICMP,
    ANY,
}

impl FirewallPolicyIntrusionDetectionProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TCP => "TCP",
            Self::UDP => "UDP",
            Self::ICMP => "ICMP",
            Self::ANY => "ANY",
        }
    }
}

impl From<FirewallPolicyIntrusionDetectionProtocol> for serde_json::Value {
    fn from(v: FirewallPolicyIntrusionDetectionProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyIntrusionDetectionProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyIntrusionDetectionProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TCP" => Ok(Self::TCP),
            "UDP" => Ok(Self::UDP),
            "ICMP" => Ok(Self::ICMP),
            "ANY" => Ok(Self::ANY),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TCP", "UDP", "ICMP", "ANY"])),
        }
    }
}
