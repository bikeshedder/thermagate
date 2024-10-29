use anyhow::Result;

use clap::{Parser, Subcommand};

mod csv;
mod gen_enums;
mod gen_params;
mod model;

#[derive(Debug, Parser)] // requires `derive` feature
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GenAll,
    GenEnums,
    GenParams,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::GenAll => {
            gen_params::cmd()?;
            gen_enums::cmd()?;
            Ok(())
        }
        Command::GenParams => gen_params::cmd(),
        Command::GenEnums => gen_enums::cmd(),
    }
}
