/// Determine update behavior for changes to network groups referenced within the rules in this configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressSpaceAggregationOption {
    None,
    Manual,
}

impl AddressSpaceAggregationOption {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Manual => "Manual",
        }
    }
}

impl From<AddressSpaceAggregationOption> for serde_json::Value {
    fn from(v: AddressSpaceAggregationOption) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AddressSpaceAggregationOption {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AddressSpaceAggregationOption {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Manual" => Ok(Self::Manual),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Manual"])),
        }
    }
}
