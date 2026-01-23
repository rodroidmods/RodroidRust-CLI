use crate::error::{CliError, Result};
use crate::template::TemplateContext;
use include_dir::{Dir, DirEntry, File};
use minijinja::Environment;
use std::fs;
use std::path::{Path, PathBuf};

const PACKAGE_PATH_TOKEN: &str = "__package_path__";

pub fn render_embedded_dir(
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
                fs::create_dir_all(&dest)?;
                render_embedded_dir(subdir, base, out_dir, context, force)?;
            }
            DirEntry::File(file) => {
                write_rendered_file(&dest, file, context, force)?;
            }
        }
    }

    Ok(())
}

pub fn render_fs_dir(src: &Path, dst: &Path, context: &TemplateContext, force: bool) -> Result<()> {
    if !src.is_dir() {
        return Err(CliError::InvalidTemplatePath(src.to_path_buf()));
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
            fs::create_dir_all(&dest)?;
            render_fs_dir(&path, &dest, context, force)?;
        } else if file_type.is_file() {
            write_rendered_file_fs(&dest, &path, context, force)?;
        }
    }

    Ok(())
}

fn write_rendered_file(
    dest: &Path,
    file: &File,
    context: &TemplateContext,
    force: bool,
) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    if dest.exists() && !force {
        return Err(CliError::FileExists(dest.to_path_buf()));
    }

    let bytes = file.contents();
    if let Ok(text) = std::str::from_utf8(bytes) {
        let rendered = render_text_if_needed(text, context)?;
        fs::write(dest, rendered)?;
    } else {
        fs::write(dest, bytes)?;
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
        fs::create_dir_all(parent)?;
    }
    if dest.exists() && !force {
        return Err(CliError::FileExists(dest.to_path_buf()));
    }

    let bytes = fs::read(src)?;
    if let Ok(text) = std::str::from_utf8(&bytes) {
        let rendered = render_text_if_needed(text, context)?;
        fs::write(dest, rendered)?;
    } else {
        fs::copy(src, dest)?;
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
        .map_err(|e| CliError::RenderError(e.to_string()))?;
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
        } else if part_str == "Cargo.toml.template" {
            out.push("Cargo.toml");
        } else {
            out.push(part);
        }
    }

    out
}
