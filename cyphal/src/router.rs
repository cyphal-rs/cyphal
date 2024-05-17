use crate::{CyphalResult, NodeId, Priority, Response, ServiceId, SubjectId};

///
pub trait Router<Su, Se, N>
where
    Su: SubjectId,
    Se: ServiceId,
    N: NodeId,
{
    ///
    async fn process_message(&self, priority: Priority, subject: Su, source: N, data: &[u8]);

    ///
    async fn process_request(
        &self,
        priority: Priority,
        service: Se,
        source: N,
        destination: N,
        data: &[u8],
    );
}
