/// The type of action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyFilterRuleCollectionActionType {
    Allow,
    Deny,
}

impl FirewallPolicyFilterRuleCollectionActionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allow => "Allow",
            Self::Deny => "Deny",
        }
    }
}

impl From<FirewallPolicyFilterRuleCollectionActionType> for serde_json::Value {
    fn from(v: FirewallPolicyFilterRuleCollectionActionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyFilterRuleCollectionActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyFilterRuleCollectionActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Allow" => Ok(Self::Allow),
            "Deny" => Ok(Self::Deny),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Allow", "Deny"])),
        }
    }
}
