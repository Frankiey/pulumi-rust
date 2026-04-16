/// The type of the rule collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallPolicyRuleCollectionType {
    FirewallPolicyNatRuleCollection,
    FirewallPolicyFilterRuleCollection,
}

impl FirewallPolicyRuleCollectionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FirewallPolicyNatRuleCollection => "FirewallPolicyNatRuleCollection",
            Self::FirewallPolicyFilterRuleCollection => "FirewallPolicyFilterRuleCollection",
        }
    }
}

impl From<FirewallPolicyRuleCollectionType> for serde_json::Value {
    fn from(v: FirewallPolicyRuleCollectionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FirewallPolicyRuleCollectionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FirewallPolicyRuleCollectionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "FirewallPolicyNatRuleCollection" => Ok(Self::FirewallPolicyNatRuleCollection),
            "FirewallPolicyFilterRuleCollection" => Ok(Self::FirewallPolicyFilterRuleCollection),
            _ => Err(serde::de::Error::unknown_variant(&s, &["FirewallPolicyNatRuleCollection", "FirewallPolicyFilterRuleCollection"])),
        }
    }
}
