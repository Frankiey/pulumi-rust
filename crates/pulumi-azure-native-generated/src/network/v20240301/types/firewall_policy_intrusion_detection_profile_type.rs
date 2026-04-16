/// IDPS profile name. When attached to a parent policy, the firewall's effective profile is the profile name of the parent policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyIntrusionDetectionProfileType {
    Basic,
    Standard,
    Advanced,
    Extended,
}

impl FirewallPolicyIntrusionDetectionProfileType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Standard => "Standard",
            Self::Advanced => "Advanced",
            Self::Extended => "Extended",
        }
    }
}

impl From<FirewallPolicyIntrusionDetectionProfileType> for serde_json::Value {
    fn from(v: FirewallPolicyIntrusionDetectionProfileType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyIntrusionDetectionProfileType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyIntrusionDetectionProfileType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Basic" => Ok(Self::Basic),
            "Standard" => Ok(Self::Standard),
            "Advanced" => Ok(Self::Advanced),
            "Extended" => Ok(Self::Extended),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Basic", "Standard", "Advanced", "Extended"])),
        }
    }
}
