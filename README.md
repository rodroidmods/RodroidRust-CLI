# Android Rust CLI

A command-line tool for scaffolding Android and Kotlin Multiplatform projects with Rust integration. Generate production-ready project templates with customizable package names, JNI bindings, and iOS C FFI support.

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
- **Multiple Templates** — Standard, multi-module, and KMP (Kotlin Multiplatform) layouts
- **KMP Support** — Kotlin Multiplatform template with Rust integration for both Android (JNI) and iOS (C FFI)
- **Interactive Prompts** — Project name, template, and package name are prompted interactively
- **Smart Package Management** — Automatic package name substitution across Kotlin, Rust, Gradle, and Xcode configs
- **Environment Diagnostics** — `doctor` command checks your toolchain setup
- **Shell Completions** — Full support for bash, zsh, fish, and PowerShell
- **Dry Run Mode** — Preview what will be created before writing files
- **Custom Templates** — Bring your own templates from any directory
- **JNI Convention Handling** — Automatic package name encoding for JNI bindings
- **jni 0.22 Support** — All templates use the latest jni crate with `EnvUnowned` and error policies

## Installation

> [!IMPORTANT]
> Please use version **0.3.2** or later for KMP template support and interactive project name prompts.

```bash
cargo install android-rust-cli
```

## Quick Start

```bash
# Create a new project (fully interactive — prompts for name, template, package)
android-rust new

# Create with a specific name
android-rust new my-app

# Initialize in current directory
android-rust init

# Specify everything via flags (non-interactive)
android-rust new my-app --template kmp-mobile --package-name com.example.app

# Preview without creating files
android-rust new my-app --dry-run

# See available templates with descriptions
android-rust list-templates
```

## Commands

### `new` — Create a New Project

```bash
android-rust new [NAME] [OPTIONS]
```

When `NAME` is omitted, the CLI will interactively prompt for a project name.

| Option | Description |
|--------|-------------|
| `--template <NAME>` | Template to use (standard, multi-module, kmp-mobile) |
| `--package-name <PKG>` | Android package name (e.g., com.example.app) |
| `--template-path <PATH>` | Use custom templates from directory |
| `--force` | Overwrite existing files |
| `--dry-run` | Preview without creating files |

### `init` — Initialize in Current Directory

```bash
android-rust init [OPTIONS]
```

Initialize a new project directly in the current directory.

```bash
# Init with standard template (default)
android-rust init

# Init with specific template
android-rust init --template multi-module
```

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
| `kmp-mobile` | Kotlin Multiplatform (Android + iOS) with Rust via JNI and C FFI |

### KMP Mobile Template

The `kmp-mobile` template generates a full Kotlin Multiplatform project with:

- **Compose Multiplatform** UI shared across Android and iOS
- **Rust workspace** with three crates:
  - `core` — Shared pure Rust logic
  - `android` — JNI bindings (cdylib) using jni 0.22
  - `ios` — C FFI bindings (staticlib) with header file
- **Kotlin expect/actual** pattern for `RustBridge` across platforms
- **iOS cinterop** setup with `.def` file and header includes
- **Xcode project** ready for iOS builds
- **Android Rust Gradle Plugin** integration for both Android and iOS Rust builds

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
2. Add a `template.toml` file (optional):

```toml
[template]
description = "My custom Android + Rust template"
features = ["jni", "compose", "custom"]
```

3. Use Jinja2 template syntax in files:
   - `{{ package_name }}` — Package name (e.g., com.example.app)
   - `{{ package_path }}` — Package path (e.g., com/example/app)
   - `{{ jni_package }}` — JNI-encoded package (e.g., com_example_app)
   - `{{ project_name }}` — Project/folder name (e.g., my-app)

4. Use `__package_path__` in directory names to create package structure

## Non-Interactive Mode

For CI/CD pipelines, all options can be specified via flags:

```bash
android-rust new my-app --template standard --package-name com.mycompany.app
```

Defaults when stdin is not a terminal:
- Project name: `my-project`
- Package: `dev.rodroid.rust`
- Template: `standard` (if available)

## Credits

**Rodroid Mods**