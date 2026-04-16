/// Gateway connection type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewayConnectionType {
    IPsec,
    Vnet2Vnet,
    ExpressRoute,
    VPNClient,
}

impl VirtualNetworkGatewayConnectionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPsec => "IPsec",
            Self::Vnet2Vnet => "Vnet2Vnet",
            Self::ExpressRoute => "ExpressRoute",
            Self::VPNClient => "VPNClient",
        }
    }
}

impl From<VirtualNetworkGatewayConnectionType> for serde_json::Value {
    fn from(v: VirtualNetworkGatewayConnectionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewayConnectionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewayConnectionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPsec" => Ok(Self::IPsec),
            "Vnet2Vnet" => Ok(Self::Vnet2Vnet),
            "ExpressRoute" => Ok(Self::ExpressRoute),
            "VPNClient" => Ok(Self::VPNClient),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPsec", "Vnet2Vnet", "ExpressRoute", "VPNClient"])),
        }
    }
}
