use crate::{NodeId, Priority, ServiceId, SubjectId};

/// Represents a router
pub trait Router {
    /// Processes an incoming message
    async fn process_message(
        &self,
        priority: Priority,
        subject: SubjectId,
        source: NodeId,
        data: &[u8],
    );

    /// Processes an incoming request
    async fn process_request(
        &self,
        priority: Priority,
        service: ServiceId,
        source: NodeId,
        destination: NodeId,
        data: &[u8],
    );
}
