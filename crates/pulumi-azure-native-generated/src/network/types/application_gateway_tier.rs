/// Tier of an application gateway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayTier {
    Standard,
    WAF,
    StandardV2,
    WAFV2,
    Basic,
}

impl ApplicationGatewayTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::WAF => "WAF",
            Self::StandardV2 => "Standard_v2",
            Self::WAFV2 => "WAF_v2",
            Self::Basic => "Basic",
        }
    }
}

impl From<ApplicationGatewayTier> for serde_json::Value {
    fn from(v: ApplicationGatewayTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            "WAF" => Ok(Self::WAF),
            "Standard_v2" => Ok(Self::StandardV2),
            "WAF_v2" => Ok(Self::WAFV2),
            "Basic" => Ok(Self::Basic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard", "WAF", "Standard_v2", "WAF_v2", "Basic"])),
        }
    }
}
