use clap::Args;
use std::path::PathBuf;

const MESSAGE: &str = r#"pub struct {NAME} {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    payload: [u8; {SIZE}],
}"#;

#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    /// The DSDL file used to generate code
    #[arg(value_name = "DSDL PATH")]
    dsdl_path: PathBuf,

    /// The location to output the generated code
    #[arg(value_name = "OUTPUT PATH")]
    output: Option<PathBuf>,

    /// Manually set the generated struct's name
    #[arg(short, long, value_name = "STRUCT NAME")]
    name: Option<String>,
}

impl GenerateArgs {
    pub fn execute(&self) {
        let name = match &self.name {
            Some(n) => n.clone(),
            None => "DefaultName".to_string(),
        };

        let message = MESSAGE.replace("{NAME}", &name).replace("{SIZE}", "16");

        match self.output {
            Some(_) => todo!(),
            None => println!("{}", message),
        }
    }
}
