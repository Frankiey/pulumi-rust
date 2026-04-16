/// Enable or Disable apply network policies on private end point in the subnet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkPrivateEndpointNetworkPolicies {
    Enabled,
    Disabled,
    NetworkSecurityGroupEnabled,
    RouteTableEnabled,
}

impl VirtualNetworkPrivateEndpointNetworkPolicies {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
            Self::NetworkSecurityGroupEnabled => "NetworkSecurityGroupEnabled",
            Self::RouteTableEnabled => "RouteTableEnabled",
        }
    }
}

impl From<VirtualNetworkPrivateEndpointNetworkPolicies> for serde_json::Value {
    fn from(v: VirtualNetworkPrivateEndpointNetworkPolicies) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkPrivateEndpointNetworkPolicies {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkPrivateEndpointNetworkPolicies {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            "NetworkSecurityGroupEnabled" => Ok(Self::NetworkSecurityGroupEnabled),
            "RouteTableEnabled" => Ok(Self::RouteTableEnabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Enabled", "Disabled", "NetworkSecurityGroupEnabled", "RouteTableEnabled"])),
        }
    }
}
