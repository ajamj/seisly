use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub plugin_type: String, // "fault" | "horizon"
    pub entry_point: String, // "main.py"
    pub author: Option<String>,
    pub description: Option<String>,
}

impl PluginManifest {
    pub fn from_yaml(content: &str) -> Result<Self> {
        let manifest: PluginManifest = serde_yaml::from_str(content)?;
        Ok(manifest)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manifest() {
        let yaml = r#"
name: MockHorizon
version: 0.1.0
plugin_type: horizon
entry_point: main.py
author: Seisly Team
"#;
        let manifest = PluginManifest::from_yaml(yaml).unwrap();
        assert_eq!(manifest.name, "MockHorizon");
        assert_eq!(manifest.version, "0.1.0");
        assert_eq!(manifest.plugin_type, "horizon");
        assert_eq!(manifest.entry_point, "main.py");
        assert_eq!(manifest.author.unwrap(), "Seisly Team");
    }
}
