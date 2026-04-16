/// Network intent policy based services.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkIntentPolicyBasedService {
    None,
    All,
    AllowRulesOnly,
}

impl NetworkIntentPolicyBasedService {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::All => "All",
            Self::AllowRulesOnly => "AllowRulesOnly",
        }
    }
}

impl From<NetworkIntentPolicyBasedService> for serde_json::Value {
    fn from(v: NetworkIntentPolicyBasedService) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NetworkIntentPolicyBasedService {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NetworkIntentPolicyBasedService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "All" => Ok(Self::All),
            "AllowRulesOnly" => Ok(Self::AllowRulesOnly),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "All", "AllowRulesOnly"])),
        }
    }
}
