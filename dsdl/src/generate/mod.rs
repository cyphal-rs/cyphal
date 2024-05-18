pub mod message;

use clap::{
    error::{Error, ErrorKind, Result as ClapResult},
    Args,
};
use cyphal_dsdl::Parser;
use message::MessageGenerator;
use std::path::PathBuf;

#[derive(Debug, Clone, Args)]
pub struct Generate {
    /// The DSDL file used to generate code
    #[arg()]
    path: PathBuf,

    /// The directory to output the generated code
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Manually set the generated struct's name
    #[arg(short, long)]
    name: Option<String>,
}

impl Generate {
    pub fn execute(&self) -> ClapResult<()> {
        let _parser = match Parser::new() {
            Ok(p) => p,
            Err(_) => return Err(Error::raw(ErrorKind::Io, "Could not create parser")),
        };

        let generator = MessageGenerator::new(self.name.clone())?;
        let code = generator.generate_code();

        match self.output {
            Some(_) => Err(Error::raw(ErrorKind::Io, "Not implemented")),
            None => {
                println!("{}", code);
                Ok(())
            }
        }
    }
}
