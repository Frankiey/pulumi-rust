/// The customized DDoS protection trigger rate sensitivity degrees. High: Trigger rate set with most sensitivity w.r.t. normal traffic. Default: Trigger rate set with moderate sensitivity w.r.t. normal traffic. Low: Trigger rate set with less sensitivity w.r.t. normal traffic. Relaxed: Trigger rate set with least sensitivity w.r.t. normal traffic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdosCustomPolicyTriggerSensitivityOverride {
    Relaxed,
    Low,
    Default,
    High,
}

impl DdosCustomPolicyTriggerSensitivityOverride {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relaxed => "Relaxed",
            Self::Low => "Low",
            Self::Default => "Default",
            Self::High => "High",
        }
    }
}

impl From<DdosCustomPolicyTriggerSensitivityOverride> for serde_json::Value {
    fn from(v: DdosCustomPolicyTriggerSensitivityOverride) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DdosCustomPolicyTriggerSensitivityOverride {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DdosCustomPolicyTriggerSensitivityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Relaxed" => Ok(Self::Relaxed),
            "Low" => Ok(Self::Low),
            "Default" => Ok(Self::Default),
            "High" => Ok(Self::High),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Relaxed", "Low", "Default", "High"])),
        }
    }
}
