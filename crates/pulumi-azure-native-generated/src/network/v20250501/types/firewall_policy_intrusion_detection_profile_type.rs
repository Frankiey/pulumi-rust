/// IDPS profile name. When attached to a parent policy, the firewall's effective profile is the profile name of the parent policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyIntrusionDetectionProfileType {
    Off,
    Emerging,
    Core,
    Extended,
}

impl FirewallPolicyIntrusionDetectionProfileType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Emerging => "Emerging",
            Self::Core => "Core",
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
            "Off" => Ok(Self::Off),
            "Emerging" => Ok(Self::Emerging),
            "Core" => Ok(Self::Core),
            "Extended" => Ok(Self::Extended),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Off", "Emerging", "Core", "Extended"])),
        }
    }
}
