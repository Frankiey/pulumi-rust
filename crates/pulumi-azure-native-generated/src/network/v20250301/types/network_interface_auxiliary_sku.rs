/// Auxiliary sku of Network Interface resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkInterfaceAuxiliarySku {
    None,
    A1,
    A2,
    A4,
    A8,
}

impl NetworkInterfaceAuxiliarySku {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::A1 => "A1",
            Self::A2 => "A2",
            Self::A4 => "A4",
            Self::A8 => "A8",
        }
    }
}

impl From<NetworkInterfaceAuxiliarySku> for serde_json::Value {
    fn from(v: NetworkInterfaceAuxiliarySku) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NetworkInterfaceAuxiliarySku {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NetworkInterfaceAuxiliarySku {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "A1" => Ok(Self::A1),
            "A2" => Ok(Self::A2),
            "A4" => Ok(Self::A4),
            "A8" => Ok(Self::A8),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "A1", "A2", "A4", "A8"])),
        }
    }
}
