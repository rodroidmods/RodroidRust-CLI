use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use include_dir::{include_dir, Dir, DirEntry, File};
use minijinja::Environment;
use serde::Serialize;
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
const DEFAULT_PACKAGE: &str = "dev.rodroid.rust";
const PACKAGE_PATH_TOKEN: &str = "__package_path__";

#[derive(Parser)]
#[command(name = "android-rust", version, about = "Generate Android + Rust JNI templates", long_about = "Generate Android + Rust JNI templates. Use the new command to create a project and list-templates to see available templates.", after_help = "Examples:\n  android-rust new my-app\n  android-rust new my-app --template multi-module\n  android-rust list-templates\n\nnew options:\n  --template <TEMPLATE>\n  --template-path <PATH>\n  --package-name <NAME>\n  --force")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "init", about = "Create a new project from a template")]
    New {
        #[arg(value_name = "NAME", help = "Target directory name (default: android-rust-example)")]
        name: Option<PathBuf>,
        #[arg(long, value_name = "TEMPLATE", help = "Template name to use (see list-templates)")]
        template: Option<String>,
        #[arg(long = "template-path", help = "Use templates from a custom directory")]
        template_path: Option<PathBuf>,
        #[arg(long = "package-name", help = "Android package/namespace (e.g. com.example.app)")]
        package_name: Option<String>,
        #[arg(long, help = "Overwrite files if the target directory is not empty")]
        force: bool,
    },
    #[command(about = "List available templates")]
    ListTemplates {
        #[arg(long = "template-path", help = "Use templates from a custom directory")]
        template_path: Option<PathBuf>,
    },
}

enum TemplateSource {
    Embedded(String),
    External { root: PathBuf, name: String },
}

#[derive(Serialize, Clone)]
struct TemplateContext {
    package_name: String,
    package_path: String,
    jni_package: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            template,
            template_path,
            package_name,
            force,
        } => new_project(name, template, template_path, package_name, force),
        Commands::ListTemplates { template_path } => list_templates_cmd(template_path),
    }
}

fn new_project(
    name: Option<PathBuf>,
    template: Option<String>,
    template_path: Option<PathBuf>,
    package_name: Option<String>,
    force: bool,
) -> Result<()> {
    let out_dir = name.unwrap_or_else(|| PathBuf::from("android-rust-example"));

    if out_dir.exists() {
        if !out_dir.is_dir() {
            bail!("Target exists and is not a directory: {}", out_dir.display());
        }
        if !force && !is_dir_empty(&out_dir)? {
            bail!(
                "Target directory is not empty: {} (use --force to overwrite)",
                out_dir.display()
            );
        }
    } else {
        fs::create_dir_all(&out_dir)
            .with_context(|| format!("Failed to create {}", out_dir.display()))?;
    }

    let template_source = resolve_template(template, template_path)?;
    let package_name = resolve_package_name(package_name)?;
    let context = TemplateContext {
        package_path: package_name.replace('.', "/"),
        jni_package: encode_jni_package(&package_name),
        package_name,
    };

    match template_source {
        TemplateSource::Embedded(name) => {
            let dir = TEMPLATE_DIR
                .get_dir(&name)
                .ok_or_else(|| anyhow!("Template not found: {}", name))?;
            render_embedded_dir(dir, dir.path(), &out_dir, &context, force)?;
        }
        TemplateSource::External { root, name } => {
            let src = root.join(&name);
            render_fs_dir(&src, &out_dir, &context, force)?;
        }
    }

    println!("Created project at {}", out_dir.display());
    Ok(())
}

fn list_templates_cmd(template_path: Option<PathBuf>) -> Result<()> {
    let names = if let Some(path) = template_path {
        list_templates_fs(&path)?
    } else {
        list_templates_embedded()
    };

    if names.is_empty() {
        bail!("No templates found");
    }

    for name in names {
        println!("{}", name);
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
            bail!("Template not found: {}", name);
        }
        return Ok(name);
    }

    if names.is_empty() {
        bail!("No templates found");
    }

    if names.len() == 1 {
        return Ok(names[0].clone());
    }

    if std::io::stdin().is_terminal() {
        let selection = Select::new()
            .with_prompt("Choose a template")
            .items(names)
            .default(0)
            .interact()?;
        return Ok(names[selection].clone());
    }

    if names.iter().any(|t| t == "standard") {
        return Ok("standard".to_string());
    }

    bail!("Multiple templates available; pass --template")
}

fn list_templates_embedded() -> Vec<String> {
    let mut names = TEMPLATE_DIR
        .dirs()
        .map(|d| d.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn list_templates_fs(path: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path)
        .with_context(|| format!("Failed to read {}", path.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}

fn resolve_package_name(package_name: Option<String>) -> Result<String> {
    if let Some(name) = package_name {
        validate_package_name(&name)?;
        return Ok(name);
    }

    if std::io::stdin().is_terminal() {
        let name: String = Input::new()
            .with_prompt("Package name")
            .default(DEFAULT_PACKAGE.to_string())
            .interact_text()?;
        validate_package_name(&name)?;
        return Ok(name);
    }

    Ok(DEFAULT_PACKAGE.to_string())
}

fn validate_package_name(name: &str) -> Result<()> {
    let is_valid = name
        .split('.')
        .all(|part| !part.is_empty() && is_valid_identifier(part));
    if !is_valid {
        bail!("Invalid package name: {}", name);
    }
    Ok(())
}

fn is_valid_identifier(part: &str) -> bool {
    let mut chars = part.chars();
    let Some(first) = chars.next() else { return false };
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn encode_jni_package(package: &str) -> String {
    package.replace('_', "_1").replace('.', "_")
}

fn render_embedded_dir(
    dir: &Dir,
    base: &Path,
    out_dir: &Path,
    context: &TemplateContext,
    force: bool,
) -> Result<()> {
    for entry in dir.entries() {
        let relative = entry
            .path()
            .strip_prefix(base)
            .unwrap_or_else(|_| entry.path());
        let dest = out_dir.join(rewrite_relative_path(relative, &context.package_path));

        match entry {
            DirEntry::Dir(subdir) => {
                fs::create_dir_all(&dest)
                    .with_context(|| format!("Failed to create directory {}", dest.display()))?;
                render_embedded_dir(subdir, base, out_dir, context, force)?;
            }
            DirEntry::File(file) => {
                write_rendered_file(&dest, file, context, force)?;
            }
        }
    }

    Ok(())
}

fn render_fs_dir(src: &Path, dst: &Path, context: &TemplateContext, force: bool) -> Result<()> {
    if !src.is_dir() {
        bail!("Template path is not a directory: {}", src.display());
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest = dst.join(rewrite_relative_path(
            entry.file_name().to_string_lossy().as_ref(),
            &context.package_path,
        ));
        let path = entry.path();

        if file_type.is_dir() {
            fs::create_dir_all(&dest)
                .with_context(|| format!("Failed to create directory {}", dest.display()))?;
            render_fs_dir(&path, &dest, context, force)?;
        } else if file_type.is_file() {
            write_rendered_file_fs(&dest, &path, context, force)?;
        }
    }

    Ok(())
}

fn write_rendered_file(dest: &Path, file: &File, context: &TemplateContext, force: bool) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    if dest.exists() && !force {
        bail!("File exists: {} (use --force to overwrite)", dest.display());
    }

    let bytes = file.contents();
    if let Ok(text) = std::str::from_utf8(bytes) {
        let rendered = render_text_if_needed(text, context)?;
        fs::write(dest, rendered)
            .with_context(|| format!("Failed to write {}", dest.display()))?;
    } else {
        fs::write(dest, bytes).with_context(|| format!("Failed to write {}", dest.display()))?;
    }

    Ok(())
}

fn write_rendered_file_fs(
    dest: &Path,
    src: &Path,
    context: &TemplateContext,
    force: bool,
) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    if dest.exists() && !force {
        bail!("File exists: {} (use --force to overwrite)", dest.display());
    }

    let bytes = fs::read(src).with_context(|| format!("Failed to read {}", src.display()))?;
    if let Ok(text) = std::str::from_utf8(&bytes) {
        let rendered = render_text_if_needed(text, context)?;
        fs::write(dest, rendered)
            .with_context(|| format!("Failed to write {}", dest.display()))?;
    } else {
        fs::copy(src, dest).with_context(|| format!("Failed to copy {}", src.display()))?;
    }

    Ok(())
}

fn render_text_if_needed(text: &str, context: &TemplateContext) -> Result<Vec<u8>> {
    if !text.contains("{{") && !text.contains("{%") {
        return Ok(text.as_bytes().to_vec());
    }

    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    let rendered = env
        .render_str(text, context)
        .with_context(|| "Failed to render template")?;
    Ok(rendered.into_bytes())
}

fn rewrite_relative_path<P: AsRef<Path>>(path: P, package_path: &str) -> PathBuf {
    let path = path.as_ref();
    let mut out = PathBuf::new();

    for part in path.iter() {
        let part_str = part.to_string_lossy();
        if part_str == PACKAGE_PATH_TOKEN {
            for sub in package_path.split('/') {
                out.push(sub);
            }
        } else {
            out.push(part);
        }
    }

    out
}

fn is_dir_empty(path: &Path) -> Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}



