/// Group connectivity type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupConnectivity {
    None,
    DirectlyConnected,
}

impl GroupConnectivity {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::DirectlyConnected => "DirectlyConnected",
        }
    }
}

impl From<GroupConnectivity> for serde_json::Value {
    fn from(v: GroupConnectivity) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for GroupConnectivity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GroupConnectivity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "DirectlyConnected" => Ok(Self::DirectlyConnected),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "DirectlyConnected"])),
        }
    }
}
