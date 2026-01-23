# Android Rust CLI

A command-line tool for scaffolding Android projects with Rust JNI integration. Generate production-ready project templates with customizable package names and support for multiple architectural patterns.

## Features

- **Instant Project Setup** - Create fully-configured Android + Rust projects in seconds
- **Multiple Templates** - Choose from pre-built templates including standard, multi-module, and bottom navigation layouts
- **Smart Package Management** - Automatic package name substitution across Java/Kotlin and Rust code
- **Interactive & Scriptable** - Works both interactively with prompts and non-interactively for CI/CD pipelines
- **Custom Templates** - Bring your own templates from any directory
- **JNI Convention Handling** - Automatically handles package name encoding for JNI bindings

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

# Use a different template
android-rust new my-app --template multi-module
android-rust new my-app --template bottom-nav

# See available templates
android-rust list-templates
```

## Advanced Usage

### Custom Templates

Use your own template directories:

```bash
# List custom templates
android-rust list-templates --template-path /path/to/templates

# Generate from custom template
android-rust new my-app --template-path /path/to/templates --template my-template
```

### Non-Interactive Mode

When stdin is not a terminal (e.g., in CI/CD), the CLI will:
- Use the default `dev.rodroid.rust` package name
- Select the `standard` template if available, otherwise fail with a clear error

Override defaults explicitly:

```bash
android-rust new my-app --template standard --package-name com.mycompany.app
```

### Force Overwrite

Regenerate into an existing directory:

```bash
android-rust new my-app --force
```

## How It Works

The CLI uses template rendering to customize project scaffolds:
1. Templates contain placeholder variables like `{{ package_name }}`
2. Directory names with `__package_path__` are expanded to the package structure
3. JNI package names are automatically encoded (e.g., `com.example.app` → `com_example_app`)
4. All files are processed and written to your target directory

## Credits

**Rodroid Mods**