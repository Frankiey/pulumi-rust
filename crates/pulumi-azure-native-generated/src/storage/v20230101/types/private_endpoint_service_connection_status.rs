/// Indicates whether the connection has been Approved/Rejected/Removed by the owner of the service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}

impl PrivateEndpointServiceConnectionStatus {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Approved => "Approved",
            Self::Rejected => "Rejected",
        }
    }
}

impl From<PrivateEndpointServiceConnectionStatus> for serde_json::Value {
    fn from(v: PrivateEndpointServiceConnectionStatus) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Pending" => Ok(Self::Pending),
            "Approved" => Ok(Self::Approved),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Pending", "Approved", "Rejected"])),
        }
    }
}
