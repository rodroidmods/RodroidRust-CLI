use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TemplateMetadata {
    #[serde(default)]
    pub template: TemplateInfo,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TemplateInfo {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

impl TemplateMetadata {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn from_file(path: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        Self::from_toml(&content).ok()
    }
}
