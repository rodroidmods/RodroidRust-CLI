use crate::error::{CliError, Result};
use crate::template::{TEMPLATE_DIR, TemplateMetadata, list_templates_embedded, list_templates_fs};
use crate::ui::{self, PACKAGE};
use console::style;
use std::path::PathBuf;

pub fn list_templates(template_path: Option<PathBuf>) -> Result<()> {
    let (names, is_external) = if let Some(ref path) = template_path {
        (list_templates_fs(path)?, true)
    } else {
        (list_templates_embedded(), false)
    };

    if names.is_empty() {
        return Err(CliError::NoTemplatesFound);
    }

    ui::print_banner();
    ui::print_section_title(PACKAGE, "Available Templates");

    for name in &names {
        let metadata = get_template_metadata(name, template_path.as_ref(), is_external);

        let (description, features) = if let Some(ref m) = metadata {
            (
                m.template
                    .description
                    .clone()
                    .unwrap_or_else(|| get_fallback_description(name).to_string()),
                m.template.features.clone(),
            )
        } else {
            (get_fallback_description(name).to_string(), Vec::new())
        };

        println!("  {} {}", style("▸").cyan().bold(), style(name).bold());
        println!("      {}", style(&description).dim());

        if !features.is_empty() {
            let features_str = features
                .iter()
                .map(|f| format!("{}", style(f).magenta()))
                .collect::<Vec<_>>()
                .join(" ");
            println!("      {}", features_str);
        }
        println!();
    }

    println!(
        "  {}",
        style(format!("{} template(s) available", names.len())).dim()
    );
    println!();

    Ok(())
}

fn get_template_metadata(
    name: &str,
    template_path: Option<&PathBuf>,
    is_external: bool,
) -> Option<TemplateMetadata> {
    if is_external {
        if let Some(path) = template_path {
            let toml_path = path.join(name).join("template.toml");
            return TemplateMetadata::from_file(&toml_path);
        }
    } else {
        let toml_path = format!("{}/template.toml", name);
        if let Some(file) = TEMPLATE_DIR.get_file(&toml_path) {
            if let Ok(content) = std::str::from_utf8(file.contents()) {
                return TemplateMetadata::from_toml(content).ok();
            }
        }
    }
    None
}

fn get_fallback_description(name: &str) -> &'static str {
    match name {
        "standard" => "Basic Android + Rust JNI project with single module",
        "multi-module" => "Clean architecture with separate app, domain, and data modules",
        "bottom-nav" => "Bottom navigation layout with Jetpack Compose",
        _ => "Custom template",
    }
}
