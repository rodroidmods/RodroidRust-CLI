use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "android-rust",
    version,
    about = "Generate Android + Rust JNI templates",
    long_about = "Generate Android + Rust JNI templates. Use the new command to create a project and list-templates to see available templates.",
    after_help = "Examples:\n  android-rust new my-app\n  android-rust new my-app --template multi-module\n  android-rust list-templates\n  android-rust doctor"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new project from a template")]
    New {
        #[arg(
            value_name = "NAME",
            help = "Target directory name (default: android-rust-example)"
        )]
        name: Option<PathBuf>,

        #[arg(
            long,
            value_name = "TEMPLATE",
            help = "Template name to use (see list-templates)"
        )]
        template: Option<String>,

        #[arg(long = "template-path", help = "Use templates from a custom directory")]
        template_path: Option<PathBuf>,

        #[arg(
            long = "package-name",
            help = "Android package/namespace (e.g. com.example.app)"
        )]
        package_name: Option<String>,

        #[arg(long, help = "Overwrite files if the target directory is not empty")]
        force: bool,

        #[arg(long, help = "Show what would be created without writing files")]
        dry_run: bool,
    },

    #[command(about = "Initialize project in current directory")]
    Init {
        #[arg(
            long,
            default_value = "standard",
            value_name = "TEMPLATE",
            help = "Template name to use (default: standard)"
        )]
        template: Option<String>,

        #[arg(long = "template-path", help = "Use templates from a custom directory")]
        template_path: Option<PathBuf>,

        #[arg(
            long = "package-name",
            help = "Android package/namespace (e.g. com.example.app)"
        )]
        package_name: Option<String>,

        #[arg(long, help = "Overwrite files if the current directory is not empty")]
        force: bool,

        #[arg(long, help = "Show what would be created without writing files")]
        dry_run: bool,
    },

    #[command(about = "List available templates")]
    ListTemplates {
        #[arg(long = "template-path", help = "Use templates from a custom directory")]
        template_path: Option<PathBuf>,
    },

    #[command(about = "Check environment and dependencies")]
    Doctor,

    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
