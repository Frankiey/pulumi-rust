/// The DDoS protection mode of the public IP
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdosSettingsProtectionMode {
    VirtualNetworkInherited,
    Enabled,
    Disabled,
}

impl DdosSettingsProtectionMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VirtualNetworkInherited => "VirtualNetworkInherited",
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl From<DdosSettingsProtectionMode> for serde_json::Value {
    fn from(v: DdosSettingsProtectionMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DdosSettingsProtectionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DdosSettingsProtectionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "VirtualNetworkInherited" => Ok(Self::VirtualNetworkInherited),
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["VirtualNetworkInherited", "Enabled", "Disabled"])),
        }
    }
}
