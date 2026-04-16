/// Name of Ssl predefined policy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewaySslPolicyName {
    AppGwSslPolicy20150501,
    AppGwSslPolicy20170401,
    AppGwSslPolicy20170401S,
}

impl ApplicationGatewaySslPolicyName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AppGwSslPolicy20150501 => "AppGwSslPolicy20150501",
            Self::AppGwSslPolicy20170401 => "AppGwSslPolicy20170401",
            Self::AppGwSslPolicy20170401S => "AppGwSslPolicy20170401S",
        }
    }
}

impl From<ApplicationGatewaySslPolicyName> for serde_json::Value {
    fn from(v: ApplicationGatewaySslPolicyName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewaySslPolicyName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewaySslPolicyName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AppGwSslPolicy20150501" => Ok(Self::AppGwSslPolicy20150501),
            "AppGwSslPolicy20170401" => Ok(Self::AppGwSslPolicy20170401),
            "AppGwSslPolicy20170401S" => Ok(Self::AppGwSslPolicy20170401S),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AppGwSslPolicy20150501", "AppGwSslPolicy20170401", "AppGwSslPolicy20170401S"])),
        }
    }
}
