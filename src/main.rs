mod cli;
mod commands;
mod error;
mod template;
mod ui;
mod validation;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            template,
            template_path,
            package_name,
            force,
            dry_run,
        } => commands::new_project(name, template, template_path, package_name, force, dry_run),
        Commands::Init {
            template,
            template_path,
            package_name,
            force,
            dry_run,
        } => commands::init_project(template, template_path, package_name, force, dry_run),
        Commands::ListTemplates { template_path } => commands::list_templates(template_path),
        Commands::Doctor => commands::run_doctor(),
        Commands::Completions { shell } => commands::generate_completions(shell),
    }
}
