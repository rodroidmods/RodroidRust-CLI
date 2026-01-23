use crate::error::{CliError, Result};
use crate::template::{
    TEMPLATE_DIR, TemplateContext, TemplateSource, list_templates_embedded, list_templates_fs,
    render_embedded_dir, render_fs_dir,
};
use crate::ui;
use crate::validation::validate_package_name;
use console::style;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_PACKAGE: &str = "dev.rodroid.rust";

pub fn new_project(
    name: Option<PathBuf>,
    template: Option<String>,
    template_path: Option<PathBuf>,
    package_name: Option<String>,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    ui::print_banner();

    let out_dir = name.unwrap_or_else(|| PathBuf::from("android-rust-example"));

    if !dry_run {
        prepare_output_dir(&out_dir, force)?;
    }

    let template_source = resolve_template(template, template_path)?;
    let package_name = resolve_package_name(package_name)?;
    let context = TemplateContext::new(package_name.clone());

    if dry_run {
        ui::print_dry_run_header();
        println!();
        ui::print_key_value("Directory", &out_dir.display().to_string());
        ui::print_key_value("Template", template_source.name());
        ui::print_key_value("Package", &package_name);
        println!();
        return Ok(());
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message(format!(
        "Creating project with {} template...",
        style(template_source.name()).cyan()
    ));

    match template_source {
        TemplateSource::Embedded(name) => {
            let dir = TEMPLATE_DIR
                .get_dir(&name)
                .ok_or_else(|| CliError::TemplateNotFound {
                    name: name.clone(),
                    available: list_templates_embedded(),
                })?;
            render_embedded_dir(dir, dir.path(), &out_dir, &context, force)?;
        }
        TemplateSource::External { root, name } => {
            let src = root.join(&name);
            render_fs_dir(&src, &out_dir, &context, force)?;
        }
    }

    pb.finish_and_clear();

    let canonical_path = out_dir.canonicalize().unwrap_or(out_dir.clone());
    let display_path = canonical_path.to_string_lossy().replace(r"\\?\", "");
    ui::print_completion_message(&display_path, &package_name);

    Ok(())
}

fn prepare_output_dir(out_dir: &PathBuf, force: bool) -> Result<()> {
    if out_dir.exists() {
        if !out_dir.is_dir() {
            return Err(CliError::TargetNotDirectory(out_dir.clone()));
        }
        if !force && !is_dir_empty(out_dir)? {
            return Err(CliError::DirectoryNotEmpty(out_dir.clone()));
        }
    } else {
        fs::create_dir_all(out_dir)?;
    }
    Ok(())
}

fn resolve_template(
    template: Option<String>,
    template_path: Option<PathBuf>,
) -> Result<TemplateSource> {
    if let Some(path) = template_path {
        let names = list_templates_fs(&path)?;
        let name = pick_template_name(template, &names)?;
        return Ok(TemplateSource::External { root: path, name });
    }

    let names = list_templates_embedded();
    let name = pick_template_name(template, &names)?;
    Ok(TemplateSource::Embedded(name))
}

fn pick_template_name(template: Option<String>, names: &[String]) -> Result<String> {
    if let Some(name) = template {
        if !names.iter().any(|t| t == &name) {
            return Err(CliError::TemplateNotFound {
                name,
                available: names.to_vec(),
            });
        }
        return Ok(name);
    }

    if names.is_empty() {
        return Err(CliError::NoTemplatesFound);
    }

    if names.len() == 1 {
        return Ok(names[0].clone());
    }

    if std::io::stdin().is_terminal() {
        println!();
        let selection = Select::new()
            .with_prompt(format!("  {} Select a template", style("?").cyan().bold()))
            .items(names)
            .default(0)
            .interact()
            .map_err(|e| CliError::Other(e.into()))?;
        return Ok(names[selection].clone());
    }

    if names.iter().any(|t| t == "standard") {
        return Ok("standard".to_string());
    }

    Err(CliError::AmbiguousTemplate)
}

fn resolve_package_name(package_name: Option<String>) -> Result<String> {
    if let Some(name) = package_name {
        validate_package_name(&name)?;
        return Ok(name);
    }

    if std::io::stdin().is_terminal() {
        println!();
        let name: String = Input::new()
            .with_prompt(format!("  {} Package name", style("?").cyan().bold()))
            .default(DEFAULT_PACKAGE.to_string())
            .interact_text()
            .map_err(|e| CliError::Other(e.into()))?;
        validate_package_name(&name)?;
        return Ok(name);
    }

    Ok(DEFAULT_PACKAGE.to_string())
}

fn is_dir_empty(path: &PathBuf) -> Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}
