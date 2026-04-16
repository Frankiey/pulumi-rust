/// Defines how a private IP address is assigned. Possible values are: 'Static' and 'Dynamic'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IPAllocationMethod {
    Static,
    Dynamic,
}

impl IPAllocationMethod {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::Dynamic => "Dynamic",
        }
    }
}

impl From<IPAllocationMethod> for serde_json::Value {
    fn from(v: IPAllocationMethod) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IPAllocationMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IPAllocationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Static" => Ok(Self::Static),
            "Dynamic" => Ok(Self::Dynamic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Static", "Dynamic"])),
        }
    }
}
