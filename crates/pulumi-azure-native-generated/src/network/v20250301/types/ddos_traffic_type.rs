/// The traffic type (one of Tcp, Udp, TcpSyn) that the detection rule will be applied upon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdosTrafficType {
    Tcp,
    Udp,
    TcpSyn,
}

impl DdosTrafficType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "Tcp",
            Self::Udp => "Udp",
            Self::TcpSyn => "TcpSyn",
        }
    }
}

impl From<DdosTrafficType> for serde_json::Value {
    fn from(v: DdosTrafficType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DdosTrafficType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DdosTrafficType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tcp" => Ok(Self::Tcp),
            "Udp" => Ok(Self::Udp),
            "TcpSyn" => Ok(Self::TcpSyn),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tcp", "Udp", "TcpSyn"])),
        }
    }
}
