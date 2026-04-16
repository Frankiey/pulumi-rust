/// VPN authentication types enabled for the VpnServerConfiguration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnAuthenticationType {
    Certificate,
    Radius,
    AAD,
}

impl VpnAuthenticationType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Certificate => "Certificate",
            Self::Radius => "Radius",
            Self::AAD => "AAD",
        }
    }
}

impl From<VpnAuthenticationType> for serde_json::Value {
    fn from(v: VpnAuthenticationType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnAuthenticationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnAuthenticationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Certificate" => Ok(Self::Certificate),
            "Radius" => Ok(Self::Radius),
            "AAD" => Ok(Self::AAD),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Certificate", "Radius", "AAD"])),
        }
    }
}
