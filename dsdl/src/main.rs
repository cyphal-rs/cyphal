mod generate;
use generate::GenerateArgs;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dsdl")]
#[command(bin_name = "dsdl")]
#[command(
    about = "Cyphal DSDL CLI",
    long_about = "Rust CLI for OpenCyphal Data Structure Description Language (DSDL) files"
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
    Generate(GenerateArgs),
}

fn main() {
    let cli = DsdlCli::parse();

    match cli.command {
        Commands::Generate(a) => a.execute(),
    }
}
