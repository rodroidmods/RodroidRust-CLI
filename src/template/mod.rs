mod context;
mod metadata;
mod renderer;
mod source;

pub use context::TemplateContext;
pub use metadata::TemplateMetadata;
pub use renderer::{render_embedded_dir, render_fs_dir};
pub use source::{TEMPLATE_DIR, TemplateSource, list_templates_embedded, list_templates_fs};
