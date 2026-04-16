/// Protocol of gateway load balancer tunnel interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayLoadBalancerTunnelProtocol {
    None,
    Native,
    VXLAN,
}

impl GatewayLoadBalancerTunnelProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Native => "Native",
            Self::VXLAN => "VXLAN",
        }
    }
}

impl From<GatewayLoadBalancerTunnelProtocol> for serde_json::Value {
    fn from(v: GatewayLoadBalancerTunnelProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for GatewayLoadBalancerTunnelProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GatewayLoadBalancerTunnelProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Native" => Ok(Self::Native),
            "VXLAN" => Ok(Self::VXLAN),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Native", "VXLAN"])),
        }
    }
}
