/// Gateway SKU name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewaySkuName {
    Basic,
    HighPerformance,
    Standard,
    UltraPerformance,
    VpnGw1,
    VpnGw2,
    VpnGw3,
    VpnGw1AZ,
    VpnGw2AZ,
    VpnGw3AZ,
    ErGw1AZ,
    ErGw2AZ,
    ErGw3AZ,
}

impl VirtualNetworkGatewaySkuName {
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
            Self::VpnGw1AZ => "VpnGw1AZ",
            Self::VpnGw2AZ => "VpnGw2AZ",
            Self::VpnGw3AZ => "VpnGw3AZ",
            Self::ErGw1AZ => "ErGw1AZ",
            Self::ErGw2AZ => "ErGw2AZ",
            Self::ErGw3AZ => "ErGw3AZ",
        }
    }
}

impl From<VirtualNetworkGatewaySkuName> for serde_json::Value {
    fn from(v: VirtualNetworkGatewaySkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewaySkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewaySkuName {
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
            "VpnGw1AZ" => Ok(Self::VpnGw1AZ),
            "VpnGw2AZ" => Ok(Self::VpnGw2AZ),
            "VpnGw3AZ" => Ok(Self::VpnGw3AZ),
            "ErGw1AZ" => Ok(Self::ErGw1AZ),
            "ErGw2AZ" => Ok(Self::ErGw2AZ),
            "ErGw3AZ" => Ok(Self::ErGw3AZ),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Basic", "HighPerformance", "Standard", "UltraPerformance", "VpnGw1", "VpnGw2", "VpnGw3", "VpnGw1AZ", "VpnGw2AZ", "VpnGw3AZ", "ErGw1AZ", "ErGw2AZ", "ErGw3AZ"])),
        }
    }
}
