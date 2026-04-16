/// Name of an application gateway SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewaySkuName {
    StandardSmall,
    StandardMedium,
    StandardLarge,
    WAFMedium,
    WAFLarge,
    StandardV2,
    WAFV2,
}

impl ApplicationGatewaySkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StandardSmall => "Standard_Small",
            Self::StandardMedium => "Standard_Medium",
            Self::StandardLarge => "Standard_Large",
            Self::WAFMedium => "WAF_Medium",
            Self::WAFLarge => "WAF_Large",
            Self::StandardV2 => "Standard_v2",
            Self::WAFV2 => "WAF_v2",
        }
    }
}

impl From<ApplicationGatewaySkuName> for serde_json::Value {
    fn from(v: ApplicationGatewaySkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewaySkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewaySkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard_Small" => Ok(Self::StandardSmall),
            "Standard_Medium" => Ok(Self::StandardMedium),
            "Standard_Large" => Ok(Self::StandardLarge),
            "WAF_Medium" => Ok(Self::WAFMedium),
            "WAF_Large" => Ok(Self::WAFLarge),
            "Standard_v2" => Ok(Self::StandardV2),
            "WAF_v2" => Ok(Self::WAFV2),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard_Small", "Standard_Medium", "Standard_Large", "WAF_Medium", "WAF_Large", "Standard_v2", "WAF_v2"])),
        }
    }
}
