/// Rule Type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyRuleType {
    ApplicationRule,
    NetworkRule,
    NatRule,
}

impl FirewallPolicyRuleType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ApplicationRule => "ApplicationRule",
            Self::NetworkRule => "NetworkRule",
            Self::NatRule => "NatRule",
        }
    }
}

impl From<FirewallPolicyRuleType> for serde_json::Value {
    fn from(v: FirewallPolicyRuleType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyRuleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ApplicationRule" => Ok(Self::ApplicationRule),
            "NetworkRule" => Ok(Self::NetworkRule),
            "NatRule" => Ok(Self::NatRule),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ApplicationRule", "NetworkRule", "NatRule"])),
        }
    }
}
