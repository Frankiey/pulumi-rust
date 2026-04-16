/// Routing Choice defines the kind of network routing opted by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingChoice {
    MicrosoftRouting,
    InternetRouting,
}

impl RoutingChoice {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MicrosoftRouting => "MicrosoftRouting",
            Self::InternetRouting => "InternetRouting",
        }
    }
}

impl From<RoutingChoice> for serde_json::Value {
    fn from(v: RoutingChoice) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RoutingChoice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RoutingChoice {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "MicrosoftRouting" => Ok(Self::MicrosoftRouting),
            "InternetRouting" => Ok(Self::InternetRouting),
            _ => Err(serde::de::Error::unknown_variant(&s, &["MicrosoftRouting", "InternetRouting"])),
        }
    }
}
