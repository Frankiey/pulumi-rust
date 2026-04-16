/// Option indicating enforcement of peerings created by the connectivity configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeeringEnforcement {
    /// Default. Peerings created by the connectivity configuration may be modified or deleted outside of the network manager.
    Unenforced,
    /// Peerings created by the connectivity configuration will not be modifiable or deletable outside of the network manager.
    Enforced,
}

impl PeeringEnforcement {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unenforced => "Unenforced",
            Self::Enforced => "Enforced",
        }
    }
}

impl From<PeeringEnforcement> for serde_json::Value {
    fn from(v: PeeringEnforcement) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PeeringEnforcement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PeeringEnforcement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Unenforced" => Ok(Self::Unenforced),
            "Enforced" => Ok(Self::Enforced),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unenforced", "Enforced"])),
        }
    }
}
