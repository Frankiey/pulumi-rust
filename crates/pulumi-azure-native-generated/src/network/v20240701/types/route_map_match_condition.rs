/// Match condition to apply RouteMap rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteMapMatchCondition {
    Unknown,
    Contains,
    Equals,
    NotContains,
    NotEquals,
}

impl RouteMapMatchCondition {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Contains => "Contains",
            Self::Equals => "Equals",
            Self::NotContains => "NotContains",
            Self::NotEquals => "NotEquals",
        }
    }
}

impl From<RouteMapMatchCondition> for serde_json::Value {
    fn from(v: RouteMapMatchCondition) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RouteMapMatchCondition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RouteMapMatchCondition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Unknown" => Ok(Self::Unknown),
            "Contains" => Ok(Self::Contains),
            "Equals" => Ok(Self::Equals),
            "NotContains" => Ok(Self::NotContains),
            "NotEquals" => Ok(Self::NotEquals),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown", "Contains", "Equals", "NotContains", "NotEquals"])),
        }
    }
}
