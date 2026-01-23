use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Template '{name}' not found. Available: {}", available.join(", "))]
    TemplateNotFound {
        name: String,
        available: Vec<String>,
    },

    #[error("Invalid package name '{name}': {reason}")]
    InvalidPackageName { name: String, reason: String },

    #[error("Target exists and is not a directory: {0}")]
    TargetNotDirectory(PathBuf),

    #[error("Target directory not empty: {0} (use --force to overwrite)")]
    DirectoryNotEmpty(PathBuf),

    #[error("File exists: {0} (use --force to overwrite)")]
    FileExists(PathBuf),

    #[error("No templates found")]
    NoTemplatesFound,

    #[error("Multiple templates available; pass --template to select one")]
    AmbiguousTemplate,

    #[error("Template path is not a directory: {0}")]
    InvalidTemplatePath(PathBuf),

    #[error("Failed to render template: {0}")]
    RenderError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, CliError>;
