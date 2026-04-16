/// Specifies the IP version type for the private IPs of the private endpoint. If not defined, this defaults to IPv4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateEndpointIPVersionType {
    /// Indicates that the Private IPs of the private endpoint will be IPv4 only.
    IPv4,
    /// Indicates that the Private IPs of the private endpoint will be IPv6 only.
    IPv6,
    /// Indicates that the Private IPs of the private endpoint can be both IPv4 and IPv6.
    DualStack,
}

impl PrivateEndpointIPVersionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
            Self::DualStack => "DualStack",
        }
    }
}

impl From<PrivateEndpointIPVersionType> for serde_json::Value {
    fn from(v: PrivateEndpointIPVersionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PrivateEndpointIPVersionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PrivateEndpointIPVersionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPv4" => Ok(Self::IPv4),
            "IPv6" => Ok(Self::IPv6),
            "DualStack" => Ok(Self::DualStack),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPv4", "IPv6", "DualStack"])),
        }
    }
}
