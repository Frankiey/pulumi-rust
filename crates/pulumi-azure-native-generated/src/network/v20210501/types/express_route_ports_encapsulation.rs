/// Encapsulation method on physical ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRoutePortsEncapsulation {
    Dot1Q,
    QinQ,
}

impl ExpressRoutePortsEncapsulation {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dot1Q => "Dot1Q",
            Self::QinQ => "QinQ",
        }
    }
}

impl From<ExpressRoutePortsEncapsulation> for serde_json::Value {
    fn from(v: ExpressRoutePortsEncapsulation) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRoutePortsEncapsulation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRoutePortsEncapsulation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Dot1Q" => Ok(Self::Dot1Q),
            "QinQ" => Ok(Self::QinQ),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Dot1Q", "QinQ"])),
        }
    }
}
