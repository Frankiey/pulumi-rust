/// Allow, disallow, or let Network Security Perimeter configuration to evaluate public network access to Storage Account. Value is optional but if passed in, must be 'Enabled', 'Disabled' or 'SecuredByPerimeter'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
    SecuredByPerimeter,
}

impl PublicNetworkAccess {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
            Self::SecuredByPerimeter => "SecuredByPerimeter",
        }
    }
}

impl From<PublicNetworkAccess> for serde_json::Value {
    fn from(v: PublicNetworkAccess) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicNetworkAccess {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicNetworkAccess {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            "SecuredByPerimeter" => Ok(Self::SecuredByPerimeter),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Enabled", "Disabled", "SecuredByPerimeter"])),
        }
    }
}
