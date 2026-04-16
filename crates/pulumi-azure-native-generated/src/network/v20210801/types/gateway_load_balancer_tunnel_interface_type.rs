/// Traffic type of gateway load balancer tunnel interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayLoadBalancerTunnelInterfaceType {
    None,
    Internal,
    External,
}

impl GatewayLoadBalancerTunnelInterfaceType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Internal => "Internal",
            Self::External => "External",
        }
    }
}

impl From<GatewayLoadBalancerTunnelInterfaceType> for serde_json::Value {
    fn from(v: GatewayLoadBalancerTunnelInterfaceType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for GatewayLoadBalancerTunnelInterfaceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GatewayLoadBalancerTunnelInterfaceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Internal" => Ok(Self::Internal),
            "External" => Ok(Self::External),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Internal", "External"])),
        }
    }
}
