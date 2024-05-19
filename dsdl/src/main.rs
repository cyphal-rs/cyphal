#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

mod generate;
use generate::Generate;

use clap::{error::Result as ClapResult, Parser, Subcommand};

fn main() -> ClapResult<()> {
    let cli = DsdlCli::parse();

    match cli.command {
        Commands::Generate(a) => a.execute()?,
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "dsdl")]
#[command(bin_name = "dsdl")]
#[command(
    about = "CLI for OpenCyphal DSDL files",
    long_about = "Command line interface (CLI) for OpenCyphal Data Structure Description Language (DSDL) files"
)]
struct DsdlCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    #[command(
        about = "Generate Rust code from a DSDL file",
        long_about = "Generates Rust code from an OpenCyphal Data Structure Description Language (DSDL) file"
    )]
    Generate(Generate),
}
