/// Represent the current migration phase of gateway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkGatewayMigrationPhase {
    /// No migration phase set on gateway.
    None,
    /// Gateway is going through prepare migration or prepare has failed. Please see state and error details for more information.
    Prepare,
    /// Prepare succeeded on gateway.
    PrepareSucceeded,
    /// Gateway is going through execute migration or execute has failed. Please see state and error details for more information.
    Execute,
    /// Execute succeeded on gateway.
    ExecuteSucceeded,
    /// Gateway is going through commit migration or commit has failed. Please see state and error details for more information.
    Commit,
    /// Commit succeeded, represent migration is complete for the gateway.
    CommitSucceeded,
    /// Represent abort succeeded on gateway, start with prepare to retrigger migration.
    AbortSucceeded,
    /// Gateway is going through abort migration or abort has failed. Please see state and error details for more information.
    Abort,
}

impl VirtualNetworkGatewayMigrationPhase {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Prepare => "Prepare",
            Self::PrepareSucceeded => "PrepareSucceeded",
            Self::Execute => "Execute",
            Self::ExecuteSucceeded => "ExecuteSucceeded",
            Self::Commit => "Commit",
            Self::CommitSucceeded => "CommitSucceeded",
            Self::AbortSucceeded => "AbortSucceeded",
            Self::Abort => "Abort",
        }
    }
}

impl From<VirtualNetworkGatewayMigrationPhase> for serde_json::Value {
    fn from(v: VirtualNetworkGatewayMigrationPhase) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkGatewayMigrationPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkGatewayMigrationPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Prepare" => Ok(Self::Prepare),
            "PrepareSucceeded" => Ok(Self::PrepareSucceeded),
            "Execute" => Ok(Self::Execute),
            "ExecuteSucceeded" => Ok(Self::ExecuteSucceeded),
            "Commit" => Ok(Self::Commit),
            "CommitSucceeded" => Ok(Self::CommitSucceeded),
            "AbortSucceeded" => Ok(Self::AbortSucceeded),
            "Abort" => Ok(Self::Abort),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Prepare", "PrepareSucceeded", "Execute", "ExecuteSucceeded", "Commit", "CommitSucceeded", "AbortSucceeded", "Abort"])),
        }
    }
}
