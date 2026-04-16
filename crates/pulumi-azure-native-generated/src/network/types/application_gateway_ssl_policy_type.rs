/// Type of Ssl Policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewaySslPolicyType {
    Predefined,
    Custom,
    CustomV2,
}

impl ApplicationGatewaySslPolicyType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Predefined => "Predefined",
            Self::Custom => "Custom",
            Self::CustomV2 => "CustomV2",
        }
    }
}

impl From<ApplicationGatewaySslPolicyType> for serde_json::Value {
    fn from(v: ApplicationGatewaySslPolicyType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewaySslPolicyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewaySslPolicyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Predefined" => Ok(Self::Predefined),
            "Custom" => Ok(Self::Custom),
            "CustomV2" => Ok(Self::CustomV2),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Predefined", "Custom", "CustomV2"])),
        }
    }
}
