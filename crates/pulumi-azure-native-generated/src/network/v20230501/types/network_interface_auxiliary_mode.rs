/// Auxiliary mode of Network Interface resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkInterfaceAuxiliaryMode {
    None,
    MaxConnections,
    Floating,
    AcceleratedConnections,
}

impl NetworkInterfaceAuxiliaryMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::MaxConnections => "MaxConnections",
            Self::Floating => "Floating",
            Self::AcceleratedConnections => "AcceleratedConnections",
        }
    }
}

impl From<NetworkInterfaceAuxiliaryMode> for serde_json::Value {
    fn from(v: NetworkInterfaceAuxiliaryMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NetworkInterfaceAuxiliaryMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NetworkInterfaceAuxiliaryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "MaxConnections" => Ok(Self::MaxConnections),
            "Floating" => Ok(Self::Floating),
            "AcceleratedConnections" => Ok(Self::AcceleratedConnections),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "MaxConnections", "Floating", "AcceleratedConnections"])),
        }
    }
}
