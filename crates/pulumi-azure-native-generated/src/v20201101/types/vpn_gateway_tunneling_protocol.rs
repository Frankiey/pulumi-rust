/// VPN protocol enabled for the VpnServerConfiguration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnGatewayTunnelingProtocol {
    IkeV2,
    OpenVPN,
}

impl VpnGatewayTunnelingProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IkeV2 => "IkeV2",
            Self::OpenVPN => "OpenVPN",
        }
    }
}

impl From<VpnGatewayTunnelingProtocol> for serde_json::Value {
    fn from(v: VpnGatewayTunnelingProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnGatewayTunnelingProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnGatewayTunnelingProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IkeV2" => Ok(Self::IkeV2),
            "OpenVPN" => Ok(Self::OpenVPN),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IkeV2", "OpenVPN"])),
        }
    }
}
