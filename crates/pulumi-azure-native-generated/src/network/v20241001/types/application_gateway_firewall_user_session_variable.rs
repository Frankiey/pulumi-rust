/// User Session clause variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayFirewallUserSessionVariable {
    ClientAddr,
    GeoLocation,
    None,
    ClientAddrXFFHeader,
    GeoLocationXFFHeader,
}

impl ApplicationGatewayFirewallUserSessionVariable {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ClientAddr => "ClientAddr",
            Self::GeoLocation => "GeoLocation",
            Self::None => "None",
            Self::ClientAddrXFFHeader => "ClientAddrXFFHeader",
            Self::GeoLocationXFFHeader => "GeoLocationXFFHeader",
        }
    }
}

impl From<ApplicationGatewayFirewallUserSessionVariable> for serde_json::Value {
    fn from(v: ApplicationGatewayFirewallUserSessionVariable) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayFirewallUserSessionVariable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayFirewallUserSessionVariable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ClientAddr" => Ok(Self::ClientAddr),
            "GeoLocation" => Ok(Self::GeoLocation),
            "None" => Ok(Self::None),
            "ClientAddrXFFHeader" => Ok(Self::ClientAddrXFFHeader),
            "GeoLocationXFFHeader" => Ok(Self::GeoLocationXFFHeader),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ClientAddr", "GeoLocation", "None", "ClientAddrXFFHeader", "GeoLocationXFFHeader"])),
        }
    }
}
