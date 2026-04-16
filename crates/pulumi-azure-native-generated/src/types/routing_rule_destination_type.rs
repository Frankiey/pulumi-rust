/// Destination type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingRuleDestinationType {
    AddressPrefix,
    ServiceTag,
}

impl RoutingRuleDestinationType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AddressPrefix => "AddressPrefix",
            Self::ServiceTag => "ServiceTag",
        }
    }
}

impl From<RoutingRuleDestinationType> for serde_json::Value {
    fn from(v: RoutingRuleDestinationType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RoutingRuleDestinationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RoutingRuleDestinationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AddressPrefix" => Ok(Self::AddressPrefix),
            "ServiceTag" => Ok(Self::ServiceTag),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AddressPrefix", "ServiceTag"])),
        }
    }
}
