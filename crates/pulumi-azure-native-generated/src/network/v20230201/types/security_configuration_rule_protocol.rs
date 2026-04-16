/// Network protocol this rule applies to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityConfigurationRuleProtocol {
    Tcp,
    Udp,
    Icmp,
    Esp,
    Any,
    Ah,
}

impl SecurityConfigurationRuleProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "Tcp",
            Self::Udp => "Udp",
            Self::Icmp => "Icmp",
            Self::Esp => "Esp",
            Self::Any => "Any",
            Self::Ah => "Ah",
        }
    }
}

impl From<SecurityConfigurationRuleProtocol> for serde_json::Value {
    fn from(v: SecurityConfigurationRuleProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityConfigurationRuleProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityConfigurationRuleProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tcp" => Ok(Self::Tcp),
            "Udp" => Ok(Self::Udp),
            "Icmp" => Ok(Self::Icmp),
            "Esp" => Ok(Self::Esp),
            "Any" => Ok(Self::Any),
            "Ah" => Ok(Self::Ah),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tcp", "Udp", "Icmp", "Esp", "Any", "Ah"])),
        }
    }
}
