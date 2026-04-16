/// The operator to be matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebApplicationFirewallOperator {
    IPMatch,
    Equal,
    Contains,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    BeginsWith,
    EndsWith,
    Regex,
    GeoMatch,
}

impl WebApplicationFirewallOperator {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPMatch => "IPMatch",
            Self::Equal => "Equal",
            Self::Contains => "Contains",
            Self::LessThan => "LessThan",
            Self::GreaterThan => "GreaterThan",
            Self::LessThanOrEqual => "LessThanOrEqual",
            Self::GreaterThanOrEqual => "GreaterThanOrEqual",
            Self::BeginsWith => "BeginsWith",
            Self::EndsWith => "EndsWith",
            Self::Regex => "Regex",
            Self::GeoMatch => "GeoMatch",
        }
    }
}

impl From<WebApplicationFirewallOperator> for serde_json::Value {
    fn from(v: WebApplicationFirewallOperator) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for WebApplicationFirewallOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WebApplicationFirewallOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPMatch" => Ok(Self::IPMatch),
            "Equal" => Ok(Self::Equal),
            "Contains" => Ok(Self::Contains),
            "LessThan" => Ok(Self::LessThan),
            "GreaterThan" => Ok(Self::GreaterThan),
            "LessThanOrEqual" => Ok(Self::LessThanOrEqual),
            "GreaterThanOrEqual" => Ok(Self::GreaterThanOrEqual),
            "BeginsWith" => Ok(Self::BeginsWith),
            "EndsWith" => Ok(Self::EndsWith),
            "Regex" => Ok(Self::Regex),
            "GeoMatch" => Ok(Self::GeoMatch),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPMatch", "Equal", "Contains", "LessThan", "GreaterThan", "LessThanOrEqual", "GreaterThanOrEqual", "BeginsWith", "EndsWith", "Regex", "GeoMatch"])),
        }
    }
}
