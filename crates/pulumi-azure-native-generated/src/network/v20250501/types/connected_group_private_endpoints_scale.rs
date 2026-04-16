/// Option indicating the scale of private endpoints allowed in the connected group of the connectivity configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectedGroupPrivateEndpointsScale {
    /// Default. Allows for up to 2K private endpoints in the connected group.
    Standard,
    /// Allows for up to 20K private endpoints in the connected group.
    HighScale,
}

impl ConnectedGroupPrivateEndpointsScale {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::HighScale => "HighScale",
        }
    }
}

impl From<ConnectedGroupPrivateEndpointsScale> for serde_json::Value {
    fn from(v: ConnectedGroupPrivateEndpointsScale) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectedGroupPrivateEndpointsScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectedGroupPrivateEndpointsScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            "HighScale" => Ok(Self::HighScale),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard", "HighScale"])),
        }
    }
}
