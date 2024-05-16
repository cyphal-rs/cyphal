use crate::{CyphalResult, NodeId, Priority, ServiceId};

/// Represents a response returned by a service
pub trait Response<S: ServiceId, N: NodeId>: Sized {
    /// Size of the response payload
    const SIZE: usize;

    /// Constructs a new response
    fn new(
        priority: Priority,
        service: S,
        destination: N,
        source: N,
        data: &[u8],
    ) -> CyphalResult<Self>;

    /// Returns the priority level of the response
    fn priority(&self) -> Priority;

    /// Returns the Service ID where the response originates
    fn service(&self) -> S;

    /// Returns the destination Node ID where the response originates
    fn destination(&self) -> N;

    /// Returns the Node ID that requested the response
    fn source(&self) -> N;

    /// Returns the payload of the response
    fn data(&self) -> &[u8];

    /// Returns th size of the response payload
    fn size() -> usize {
        Self::SIZE
    }
}
