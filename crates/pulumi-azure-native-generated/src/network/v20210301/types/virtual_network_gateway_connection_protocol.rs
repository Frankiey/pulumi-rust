/// Connection protocol used for this connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewayConnectionProtocol {
    IKEv2,
    IKEv1,
}

impl VirtualNetworkGatewayConnectionProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IKEv2 => "IKEv2",
            Self::IKEv1 => "IKEv1",
        }
    }
}

impl From<VirtualNetworkGatewayConnectionProtocol> for serde_json::Value {
    fn from(v: VirtualNetworkGatewayConnectionProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewayConnectionProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewayConnectionProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IKEv2" => Ok(Self::IKEv2),
            "IKEv1" => Ok(Self::IKEv1),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IKEv2", "IKEv1"])),
        }
    }
}
