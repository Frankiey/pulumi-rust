/// Indicates local route setting for this particular rule collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingRuleCollectionLocalRouteSetting {
    NotSpecified,
    DirectRoutingWithinVNet,
    DirectRoutingWithinSubnet,
}

impl RoutingRuleCollectionLocalRouteSetting {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotSpecified => "NotSpecified",
            Self::DirectRoutingWithinVNet => "DirectRoutingWithinVNet",
            Self::DirectRoutingWithinSubnet => "DirectRoutingWithinSubnet",
        }
    }
}

impl From<RoutingRuleCollectionLocalRouteSetting> for serde_json::Value {
    fn from(v: RoutingRuleCollectionLocalRouteSetting) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RoutingRuleCollectionLocalRouteSetting {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RoutingRuleCollectionLocalRouteSetting {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NotSpecified" => Ok(Self::NotSpecified),
            "DirectRoutingWithinVNet" => Ok(Self::DirectRoutingWithinVNet),
            "DirectRoutingWithinSubnet" => Ok(Self::DirectRoutingWithinSubnet),
            _ => Err(serde::de::Error::unknown_variant(&s, &["NotSpecified", "DirectRoutingWithinVNet", "DirectRoutingWithinSubnet"])),
        }
    }
}
