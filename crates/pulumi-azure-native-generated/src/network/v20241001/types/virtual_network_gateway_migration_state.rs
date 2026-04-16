/// Represent the current state of gateway migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewayMigrationState {
    None,
    InProgress,
    Succeeded,
    Failed,
}

impl VirtualNetworkGatewayMigrationState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::InProgress => "InProgress",
            Self::Succeeded => "Succeeded",
            Self::Failed => "Failed",
        }
    }
}

impl From<VirtualNetworkGatewayMigrationState> for serde_json::Value {
    fn from(v: VirtualNetworkGatewayMigrationState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewayMigrationState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewayMigrationState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "InProgress" => Ok(Self::InProgress),
            "Succeeded" => Ok(Self::Succeeded),
            "Failed" => Ok(Self::Failed),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "InProgress", "Succeeded", "Failed"])),
        }
    }
}
