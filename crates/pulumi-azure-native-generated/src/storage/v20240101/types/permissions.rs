/// The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permissions {
    R,
    D,
    W,
    L,
    A,
    C,
    U,
    P,
}

impl Permissions {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R => "r",
            Self::D => "d",
            Self::W => "w",
            Self::L => "l",
            Self::A => "a",
            Self::C => "c",
            Self::U => "u",
            Self::P => "p",
        }
    }
}

impl From<Permissions> for serde_json::Value {
    fn from(v: Permissions) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Permissions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Permissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "r" => Ok(Self::R),
            "d" => Ok(Self::D),
            "w" => Ok(Self::W),
            "l" => Ok(Self::L),
            "a" => Ok(Self::A),
            "c" => Ok(Self::C),
            "u" => Ok(Self::U),
            "p" => Ok(Self::P),
            _ => Err(serde::de::Error::unknown_variant(&s, &["r", "d", "w", "l", "a", "c", "u", "p"])),
        }
    }
}
