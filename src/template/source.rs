use crate::error::{CliError, Result};
use include_dir::{Dir, include_dir};
use std::fs;
use std::path::{Path, PathBuf};

pub static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub enum TemplateSource {
    Embedded(String),
    External { root: PathBuf, name: String },
}

impl TemplateSource {
    pub fn name(&self) -> &str {
        match self {
            TemplateSource::Embedded(name) => name,
            TemplateSource::External { name, .. } => name,
        }
    }
}

pub fn list_templates_embedded() -> Vec<String> {
    let mut names: Vec<String> = TEMPLATE_DIR
        .dirs()
        .map(|d| d.path().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

pub fn list_templates_fs(path: &Path) -> Result<Vec<String>> {
    if !path.is_dir() {
        return Err(CliError::InvalidTemplatePath(path.to_path_buf()));
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}
