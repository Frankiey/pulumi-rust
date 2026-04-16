/// Determines the peering route propogation rule behavior. Defaults to 'Default'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropagationOption {
    Default,
    DisableAllPeeringPrefixes,
}

impl PropagationOption {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::DisableAllPeeringPrefixes => "DisableAllPeeringPrefixes",
        }
    }
}

impl From<PropagationOption> for serde_json::Value {
    fn from(v: PropagationOption) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PropagationOption {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PropagationOption {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Default" => Ok(Self::Default),
            "DisableAllPeeringPrefixes" => Ok(Self::DisableAllPeeringPrefixes),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Default", "DisableAllPeeringPrefixes"])),
        }
    }
}
