use clap::error::Result as ClapResult;

const MESSAGE: &str = r#"
use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId};

const MESSAGE_SIZE: usize = {SIZE};

/// Represents a {NAME} message
pub struct {NAME} {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; MESSAGE_SIZE],
}

impl {NAME} {
    pub fn raw(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        data: [u8; MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            data,
        })
    }
}

impl Message for TestSmallMessage {
    const SIZE: usize = MESSAGE_SIZE;

    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
"#;

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
