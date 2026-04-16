/// Route table usage mode defines which route table will be used by the configuration. If not defined, this will default to 'ManagedOnly'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteTableUsageMode {
    /// Only route tables managed by the routing configuration will be used.
    ManagedOnly,
    /// Use existing user-defined route tables already associated with resources.
    UseExisting,
}

impl RouteTableUsageMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ManagedOnly => "ManagedOnly",
            Self::UseExisting => "UseExisting",
        }
    }
}

impl From<RouteTableUsageMode> for serde_json::Value {
    fn from(v: RouteTableUsageMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RouteTableUsageMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RouteTableUsageMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ManagedOnly" => Ok(Self::ManagedOnly),
            "UseExisting" => Ok(Self::UseExisting),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ManagedOnly", "UseExisting"])),
        }
    }
}
