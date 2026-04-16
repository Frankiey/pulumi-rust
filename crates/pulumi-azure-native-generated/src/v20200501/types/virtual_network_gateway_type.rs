/// The type of this virtual network gateway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewayType {
    Vpn,
    ExpressRoute,
}

impl VirtualNetworkGatewayType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vpn => "Vpn",
            Self::ExpressRoute => "ExpressRoute",
        }
    }
}

impl From<VirtualNetworkGatewayType> for serde_json::Value {
    fn from(v: VirtualNetworkGatewayType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewayType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewayType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Vpn" => Ok(Self::Vpn),
            "ExpressRoute" => Ok(Self::ExpressRoute),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Vpn", "ExpressRoute"])),
        }
    }
}
