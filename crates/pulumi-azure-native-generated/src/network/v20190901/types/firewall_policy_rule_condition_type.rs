/// Rule Condition Type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyRuleConditionType {
    ApplicationRuleCondition,
    NetworkRuleCondition,
}

impl FirewallPolicyRuleConditionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ApplicationRuleCondition => "ApplicationRuleCondition",
            Self::NetworkRuleCondition => "NetworkRuleCondition",
        }
    }
}

impl From<FirewallPolicyRuleConditionType> for serde_json::Value {
    fn from(v: FirewallPolicyRuleConditionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyRuleConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyRuleConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ApplicationRuleCondition" => Ok(Self::ApplicationRuleCondition),
            "NetworkRuleCondition" => Ok(Self::NetworkRuleCondition),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ApplicationRuleCondition", "NetworkRuleCondition"])),
        }
    }
}
