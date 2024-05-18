use clap::error::Result as ClapResult;

const MESSAGE: &str = r#"pub struct {NAME} {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    payload: [u8; {SIZE}],
}"#;

pub struct MessageGenerator {
    name: Option<String>,
}

impl MessageGenerator {
    pub fn new(name: Option<String>) -> ClapResult<Self> {
        Ok(Self { name })
    }

    pub fn generate_code(&self) -> String {
        let name = match &self.name {
            Some(n) => n.clone(),
            None => "DefaultName".to_string(),
        };

        MESSAGE
            .replace("{NAME}", &name)
            .replace("{SIZE}", "16")
            .to_string()
    }
}
