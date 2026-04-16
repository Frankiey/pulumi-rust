/// Duration over which Rate Limit policy will be applied. Applies only when ruleType is RateLimitRule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayFirewallRateLimitDuration {
    OneMin,
    FiveMins,
}

impl ApplicationGatewayFirewallRateLimitDuration {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneMin => "OneMin",
            Self::FiveMins => "FiveMins",
        }
    }
}

impl From<ApplicationGatewayFirewallRateLimitDuration> for serde_json::Value {
    fn from(v: ApplicationGatewayFirewallRateLimitDuration) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayFirewallRateLimitDuration {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayFirewallRateLimitDuration {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "OneMin" => Ok(Self::OneMin),
            "FiveMins" => Ok(Self::FiveMins),
            _ => Err(serde::de::Error::unknown_variant(&s, &["OneMin", "FiveMins"])),
        }
    }
}
