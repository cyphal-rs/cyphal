use crate::{NodeId, Priority, ServiceId, SubjectId};

/// Represents a router
pub trait Router<Su, Se, N>
where
    Su: SubjectId,
    Se: ServiceId,
    N: NodeId,
{
    /// Processes an incoming message
    async fn process_message(&self, priority: Priority, subject: Su, source: N, data: &[u8]);

    /// Processes an incoming request
    async fn process_request(
        &self,
        priority: Priority,
        service: Se,
        source: N,
        destination: N,
        data: &[u8],
    );
}
