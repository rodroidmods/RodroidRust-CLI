use crate::cli::Cli;
use crate::error::Result;
use clap::CommandFactory;
use clap_complete::Shell;
use std::io;

pub fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "android-rust", &mut io::stdout());
    Ok(())
}
