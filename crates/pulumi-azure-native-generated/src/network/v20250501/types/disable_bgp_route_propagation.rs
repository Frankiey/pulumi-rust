/// Determines whether BGP route propagation is enabled. Defaults to true.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisableBgpRoutePropagation {
    /// BGP route propagation is enabled.
    False,
    /// BGP route propagation is disabled.
    True,
}

impl DisableBgpRoutePropagation {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::False => "False",
            Self::True => "True",
        }
    }
}

impl From<DisableBgpRoutePropagation> for serde_json::Value {
    fn from(v: DisableBgpRoutePropagation) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DisableBgpRoutePropagation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DisableBgpRoutePropagation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "False" => Ok(Self::False),
            "True" => Ok(Self::True),
            _ => Err(serde::de::Error::unknown_variant(&s, &["False", "True"])),
        }
    }
}
