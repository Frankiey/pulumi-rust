/// The type for the IpAllocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpAllocationType {
    Undefined,
    Hypernet,
}

impl IpAllocationType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Undefined => "Undefined",
            Self::Hypernet => "Hypernet",
        }
    }
}

impl From<IpAllocationType> for serde_json::Value {
    fn from(v: IpAllocationType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IpAllocationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IpAllocationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Undefined" => Ok(Self::Undefined),
            "Hypernet" => Ok(Self::Hypernet),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Undefined", "Hypernet"])),
        }
    }
}
