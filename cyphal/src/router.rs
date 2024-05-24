use crate::{CyphalResult, NodeId, Priority, Response, ServiceId, SubjectId};

/// Represents a router
pub trait Router {
    /// Processes an incoming message
    #[allow(unused_variables)]
    async fn process_message(
        &self,
        priority: Priority,
        subject: SubjectId,
        source: NodeId,
        data: &[u8],
    ) -> CyphalResult<()> {
        Ok(())
    }

    /// Processes an incoming request
    #[allow(unused_variables)]
    async fn process_request(
        &self,
        priority: Priority,
        service: ServiceId,
        source: NodeId,
        destination: NodeId,
        data: &[u8],
    ) -> CyphalResult<Option<impl Response>> {
        struct NullResponse {}

        impl Response for NullResponse {
            const SIZE: usize = 0;

            fn new_raw(
                _priority: Priority,
                _service: ServiceId,
                _source: NodeId,
                _destination: NodeId,
                _data: &[u8],
            ) -> CyphalResult<Self> {
                Err(crate::CyphalError::Transport)
            }

            fn priority(&self) -> Priority {
                Priority::Optional
            }

            fn service(&self) -> ServiceId {
                0
            }

            fn destination(&self) -> NodeId {
                0
            }

            fn source(&self) -> NodeId {
                0
            }

            fn data(&self) -> &[u8] {
                &[0]
            }
        }

        Ok(None::<NullResponse>)
    }
}
