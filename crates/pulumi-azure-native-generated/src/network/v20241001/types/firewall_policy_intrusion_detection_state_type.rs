/// Intrusion detection general state. When attached to a parent policy, the firewall's effective IDPS mode is the stricter mode of the two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyIntrusionDetectionStateType {
    Off,
    Alert,
    Deny,
}

impl FirewallPolicyIntrusionDetectionStateType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Alert => "Alert",
            Self::Deny => "Deny",
        }
    }
}

impl From<FirewallPolicyIntrusionDetectionStateType> for serde_json::Value {
    fn from(v: FirewallPolicyIntrusionDetectionStateType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyIntrusionDetectionStateType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyIntrusionDetectionStateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Off" => Ok(Self::Off),
            "Alert" => Ok(Self::Alert),
            "Deny" => Ok(Self::Deny),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Off", "Alert", "Deny"])),
        }
    }
}
