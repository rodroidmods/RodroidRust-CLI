# Android Rust CLI

A command-line tool for scaffolding Android projects with Rust JNI integration. Generate production-ready project templates with customizable package names and support for multiple architectural patterns.

```
   ╭──────────────────────────────────────────╮
   │                                          │
   │   ╔═╗╔╗╔╔╦╗╦═╗╔═╗╦╔╦╗  ╦═╗╦ ╦╔═╗╔╦╗      │
   │   ╠═╣║║║ ║║╠╦╝║ ║║ ║║  ╠╦╝║ ║╚═╗ ║       │
   │   ╩ ╩╝╚╝═╩╝╩╚═╚═╝╩═╩╝  ╩╚═╚═╝╚═╝ ╩       │
   │                                          │
   ╰──────────────────────────────────────────╯
```

## Features

- **Instant Project Setup** — Create fully-configured Android + Rust projects in seconds
- **Multiple Templates** — Standard, multi-module, and bottom navigation layouts
- **Smart Package Management** — Automatic package name substitution across Java/Kotlin and Rust
- **Environment Diagnostics** — `doctor` command checks your toolchain setup
- **Shell Completions** — Full support for bash, zsh, fish, and PowerShell
- **Dry Run Mode** — Preview what will be created before writing files
- **Custom Templates** — Bring your own templates from any directory
- **JNI Convention Handling** — Automatic package name encoding for JNI bindings

## Installation

```bash
cargo install android-rust-cli
```

## Quick Start

```bash
# Create a new project (interactive mode)
android-rust new my-app

# Specify template and package name
android-rust new my-app --template standard --package-name com.example.app

# Preview without creating files
android-rust new my-app --dry-run

# See available templates with descriptions
android-rust list-templates
```

## Commands

### `new` — Create a New Project

```bash
android-rust new <name> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--template <NAME>` | Template to use (standard, multi-module, bottom-nav) |
| `--package-name <PKG>` | Android package name (e.g., com.example.app) |
| `--template-path <PATH>` | Use custom templates from directory |
| `--force` | Overwrite existing files |
| `--dry-run` | Preview without creating files |

### `list-templates` — View Available Templates

```bash
android-rust list-templates [--template-path <PATH>]
```

### `doctor` — Check Environment

Diagnose your development environment:

```bash
android-rust doctor
```

Checks:
- ✓ Rust toolchain version
- ✓ Android NDK installation
- ✓ cargo-ndk availability
- ✓ Required Android targets

### `completions` — Generate Shell Completions

```bash
# Bash
android-rust completions bash > ~/.local/share/bash-completion/completions/android-rust

# Zsh
android-rust completions zsh > ~/.zfunc/_android-rust

# Fish
android-rust completions fish > ~/.config/fish/completions/android-rust.fish

# PowerShell
android-rust completions powershell > android-rust.ps1
```

## Templates

| Template | Description |
|----------|-------------|
| `standard` | Basic Android + Rust JNI project with single module |
| `multi-module` | Clean architecture with app, domain, and data modules |
| `bottom-nav` | Bottom navigation layout with Jetpack Compose |

## Custom Templates

Use your own template directories:

```bash
# List custom templates
android-rust list-templates --template-path /path/to/templates

# Generate from custom template
android-rust new my-app --template-path /path/to/templates --template my-template
```

### Creating Custom Templates

1. Create a directory for your template
2. Add a `template.toml` file:

```toml
[template]
description = "My custom Android + Rust template"
features = ["jni", "compose", "custom"]
```

3. Use Jinja2 template syntax in files:
   - `{{ package_name }}` — Package name (e.g., com.example.app)
   - `{{ package_path }}` — Package path (e.g., com/example/app)
   - `{{ jni_package }}` — JNI-encoded package (e.g., com_example_app)

4. Use `__package_path__` in directory names to create package structure

## Non-Interactive Mode

For CI/CD pipelines, all options can be specified via flags:

```bash
android-rust new my-app --template standard --package-name com.mycompany.app
```

Defaults when stdin is not a terminal:
- Package: `dev.rodroid.rust`
- Template: `standard` (if available)

## Credits

**Rodroid Mods**