/// Represents a directive
#[derive(Debug, PartialEq)]
pub enum Directive {
    /// Represents the @sealed directive and contains an optional comment
    Sealed(Option<String>),
}
