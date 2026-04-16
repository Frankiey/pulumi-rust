/// Gateway SKU tier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewaySkuTier {
    Basic,
    HighPerformance,
    Standard,
    UltraPerformance,
    VpnGw1,
    VpnGw2,
    VpnGw3,
    VpnGw4,
    VpnGw5,
    VpnGw1AZ,
    VpnGw2AZ,
    VpnGw3AZ,
    VpnGw4AZ,
    VpnGw5AZ,
    ErGw1AZ,
    ErGw2AZ,
    ErGw3AZ,
    ErGwScale,
}

impl VirtualNetworkGatewaySkuTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::HighPerformance => "HighPerformance",
            Self::Standard => "Standard",
            Self::UltraPerformance => "UltraPerformance",
            Self::VpnGw1 => "VpnGw1",
            Self::VpnGw2 => "VpnGw2",
            Self::VpnGw3 => "VpnGw3",
            Self::VpnGw4 => "VpnGw4",
            Self::VpnGw5 => "VpnGw5",
            Self::VpnGw1AZ => "VpnGw1AZ",
            Self::VpnGw2AZ => "VpnGw2AZ",
            Self::VpnGw3AZ => "VpnGw3AZ",
            Self::VpnGw4AZ => "VpnGw4AZ",
            Self::VpnGw5AZ => "VpnGw5AZ",
            Self::ErGw1AZ => "ErGw1AZ",
            Self::ErGw2AZ => "ErGw2AZ",
            Self::ErGw3AZ => "ErGw3AZ",
            Self::ErGwScale => "ErGwScale",
        }
    }
}

impl From<VirtualNetworkGatewaySkuTier> for serde_json::Value {
    fn from(v: VirtualNetworkGatewaySkuTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewaySkuTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewaySkuTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Basic" => Ok(Self::Basic),
            "HighPerformance" => Ok(Self::HighPerformance),
            "Standard" => Ok(Self::Standard),
            "UltraPerformance" => Ok(Self::UltraPerformance),
            "VpnGw1" => Ok(Self::VpnGw1),
            "VpnGw2" => Ok(Self::VpnGw2),
            "VpnGw3" => Ok(Self::VpnGw3),
            "VpnGw4" => Ok(Self::VpnGw4),
            "VpnGw5" => Ok(Self::VpnGw5),
            "VpnGw1AZ" => Ok(Self::VpnGw1AZ),
            "VpnGw2AZ" => Ok(Self::VpnGw2AZ),
            "VpnGw3AZ" => Ok(Self::VpnGw3AZ),
            "VpnGw4AZ" => Ok(Self::VpnGw4AZ),
            "VpnGw5AZ" => Ok(Self::VpnGw5AZ),
            "ErGw1AZ" => Ok(Self::ErGw1AZ),
            "ErGw2AZ" => Ok(Self::ErGw2AZ),
            "ErGw3AZ" => Ok(Self::ErGw3AZ),
            "ErGwScale" => Ok(Self::ErGwScale),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Basic", "HighPerformance", "Standard", "UltraPerformance", "VpnGw1", "VpnGw2", "VpnGw3", "VpnGw4", "VpnGw5", "VpnGw1AZ", "VpnGw2AZ", "VpnGw3AZ", "VpnGw4AZ", "VpnGw5AZ", "ErGw1AZ", "ErGw2AZ", "ErGw3AZ", "ErGwScale"])),
        }
    }
}
