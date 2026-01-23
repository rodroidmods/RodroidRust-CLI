# Android Rust CLI

Generate Android + Rust JNI template projects with a single command. The CLI embeds project templates, supports multiple templates, and renders package/namespace values using a template engine.

## Usage

```bash
android-rust new my-app
android-rust new my-app --template standard
android-rust new my-app --template multi-module
android-rust new my-app --template bottom-nav
android-rust new my-app --package-name com.example.app
android-rust list-templates
```

If multiple templates are available and no template is provided, the CLI will prompt you to choose.
It will also prompt for a package name when running interactively.

## Custom templates

```bash
android-rust list-templates --template-path /path/to/templates
android-rust new my-app --template-path /path/to/templates --template my-template
```

## Credits

Rodroid Mods
