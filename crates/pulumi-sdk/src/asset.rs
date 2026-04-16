use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The special signature key used by Pulumi to encode non-trivial values.
const SIG_KEY: &str = "4dabf18193072939515e22adb298388d";

/// Signature value for an Asset.
const ASSET_SIG: &str = "c44067f5952c0a294b673a41bacd8c17";

/// Signature value for an Archive.
const ARCHIVE_SIG: &str = "0def7320c3a5731c473e5ecbe6d01bc7";

/// An asset is a blob of text or data that can be passed to a resource.
/// Assets can be created from strings, local files, or remote URLs.
#[derive(Debug, Clone, PartialEq)]
pub enum Asset {
    /// An asset whose content is a string.
    Text(String),
    /// An asset whose content is read from a file on disk.
    File(String),
    /// An asset whose content is read from a URL.
    Remote(String),
}

/// An archive is a collection of named assets or other archives.
#[derive(Debug, Clone, PartialEq)]
pub enum Archive {
    /// An archive whose contents are read from a directory on disk.
    Path(String),
    /// An archive whose contents are read from a URL (e.g., a .tar.gz or .zip).
    Remote(String),
    /// An archive built from a map of named assets/archives.
    Assets(HashMap<String, AssetOrArchive>),
}

/// A union type representing either an Asset or an Archive.
#[derive(Debug, Clone, PartialEq)]
pub enum AssetOrArchive {
    Asset(Asset),
    Archive(Archive),
}

impl Asset {
    /// Create a text asset from a string.
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    /// Create a file asset from a path.
    pub fn file(path: impl Into<String>) -> Self {
        Self::File(path.into())
    }

    /// Create a remote asset from a URL.
    pub fn remote(url: impl Into<String>) -> Self {
        Self::Remote(url.into())
    }
}

impl Archive {
    /// Create a path archive from a directory.
    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    /// Create a remote archive from a URL.
    pub fn remote(url: impl Into<String>) -> Self {
        Self::Remote(url.into())
    }

    /// Create an archive from a map of named assets/archives.
    pub fn assets(assets: HashMap<String, AssetOrArchive>) -> Self {
        Self::Assets(assets)
    }
}

impl From<Asset> for AssetOrArchive {
    fn from(a: Asset) -> Self {
        AssetOrArchive::Asset(a)
    }
}

impl From<Archive> for AssetOrArchive {
    fn from(a: Archive) -> Self {
        AssetOrArchive::Archive(a)
    }
}

// --- Serialization to Pulumi wire format ---

impl Serialize for Asset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(SIG_KEY, ASSET_SIG)?;
        match self {
            Asset::Text(s) => map.serialize_entry("text", s)?,
            Asset::File(p) => map.serialize_entry("path", p)?,
            Asset::Remote(u) => map.serialize_entry("uri", u)?,
        }
        map.end()
    }
}

impl Serialize for Archive {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(SIG_KEY, ARCHIVE_SIG)?;
        match self {
            Archive::Path(p) => map.serialize_entry("path", p)?,
            Archive::Remote(u) => map.serialize_entry("uri", u)?,
            Archive::Assets(a) => map.serialize_entry("assets", a)?,
        }
        map.end()
    }
}

impl Serialize for AssetOrArchive {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AssetOrArchive::Asset(a) => a.serialize(serializer),
            AssetOrArchive::Archive(a) => a.serialize(serializer),
        }
    }
}

// --- Deserialization from Pulumi wire format ---

impl<'de> Deserialize<'de> for Asset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;

        let sig = map
            .get(SIG_KEY)
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::missing_field(SIG_KEY))?;

        if sig != ASSET_SIG {
            return Err(serde::de::Error::custom(format!(
                "expected asset signature {ASSET_SIG}, got {sig}"
            )));
        }

        if let Some(text) = map.get("text").and_then(|v| v.as_str()) {
            Ok(Asset::Text(text.to_string()))
        } else if let Some(path) = map.get("path").and_then(|v| v.as_str()) {
            Ok(Asset::File(path.to_string()))
        } else if let Some(uri) = map.get("uri").and_then(|v| v.as_str()) {
            Ok(Asset::Remote(uri.to_string()))
        } else {
            Err(serde::de::Error::custom(
                "asset must have 'text', 'path', or 'uri' field",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Archive {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;

        let sig = map
            .get(SIG_KEY)
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::missing_field(SIG_KEY))?;

        if sig != ARCHIVE_SIG {
            return Err(serde::de::Error::custom(format!(
                "expected archive signature {ARCHIVE_SIG}, got {sig}"
            )));
        }

        if let Some(path) = map.get("path").and_then(|v| v.as_str()) {
            Ok(Archive::Path(path.to_string()))
        } else if let Some(uri) = map.get("uri").and_then(|v| v.as_str()) {
            Ok(Archive::Remote(uri.to_string()))
        } else if let Some(assets) = map.get("assets") {
            let assets: HashMap<String, AssetOrArchive> =
                serde_json::from_value(assets.clone()).map_err(serde::de::Error::custom)?;
            Ok(Archive::Assets(assets))
        } else {
            Err(serde::de::Error::custom(
                "archive must have 'path', 'uri', or 'assets' field",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for AssetOrArchive {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;

        let sig = map
            .get(SIG_KEY)
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::missing_field(SIG_KEY))?;

        // Re-serialize the map so we can deserialize into the correct variant.
        let json = serde_json::to_value(&map).map_err(serde::de::Error::custom)?;

        if sig == ASSET_SIG {
            let asset: Asset = serde_json::from_value(json).map_err(serde::de::Error::custom)?;
            Ok(AssetOrArchive::Asset(asset))
        } else if sig == ARCHIVE_SIG {
            let archive: Archive =
                serde_json::from_value(json).map_err(serde::de::Error::custom)?;
            Ok(AssetOrArchive::Archive(archive))
        } else {
            Err(serde::de::Error::custom(format!(
                "unknown asset/archive signature: {sig}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_asset_roundtrip() {
        let asset = Asset::text("hello world");
        let json = serde_json::to_value(&asset).unwrap();
        assert_eq!(json[SIG_KEY], ASSET_SIG);
        assert_eq!(json["text"], "hello world");

        let back: Asset = serde_json::from_value(json).unwrap();
        assert_eq!(back, asset);
    }

    #[test]
    fn test_file_asset_roundtrip() {
        let asset = Asset::file("/tmp/my-file.txt");
        let json = serde_json::to_value(&asset).unwrap();
        assert_eq!(json["path"], "/tmp/my-file.txt");

        let back: Asset = serde_json::from_value(json).unwrap();
        assert_eq!(back, asset);
    }

    #[test]
    fn test_remote_asset_roundtrip() {
        let asset = Asset::remote("https://example.com/file.zip");
        let json = serde_json::to_value(&asset).unwrap();
        assert_eq!(json["uri"], "https://example.com/file.zip");

        let back: Asset = serde_json::from_value(json).unwrap();
        assert_eq!(back, asset);
    }

    #[test]
    fn test_path_archive_roundtrip() {
        let archive = Archive::path("/tmp/my-dir");
        let json = serde_json::to_value(&archive).unwrap();
        assert_eq!(json[SIG_KEY], ARCHIVE_SIG);
        assert_eq!(json["path"], "/tmp/my-dir");

        let back: Archive = serde_json::from_value(json).unwrap();
        assert_eq!(back, archive);
    }

    #[test]
    fn test_remote_archive_roundtrip() {
        let archive = Archive::remote("https://example.com/archive.tar.gz");
        let json = serde_json::to_value(&archive).unwrap();
        assert_eq!(json["uri"], "https://example.com/archive.tar.gz");

        let back: Archive = serde_json::from_value(json).unwrap();
        assert_eq!(back, archive);
    }

    #[test]
    fn test_asset_archive_roundtrip() {
        let mut assets = HashMap::new();
        assets.insert("index.js".into(), Asset::text("console.log('hi')").into());
        assets.insert(
            "data.zip".into(),
            Archive::remote("https://example.com/data.zip").into(),
        );

        let archive = Archive::assets(assets);
        let json = serde_json::to_value(&archive).unwrap();
        assert_eq!(json[SIG_KEY], ARCHIVE_SIG);

        let back: Archive = serde_json::from_value(json).unwrap();
        assert_eq!(back, archive);
    }

    #[test]
    fn test_asset_or_archive_deserialize() {
        let asset = Asset::text("hello");
        let json = serde_json::to_value(&asset).unwrap();
        let aoa: AssetOrArchive = serde_json::from_value(json).unwrap();
        assert_eq!(aoa, AssetOrArchive::Asset(Asset::Text("hello".into())));

        let archive = Archive::path("/tmp");
        let json = serde_json::to_value(&archive).unwrap();
        let aoa: AssetOrArchive = serde_json::from_value(json).unwrap();
        assert_eq!(aoa, AssetOrArchive::Archive(Archive::Path("/tmp".into())));
    }
}
