use crate::{CyphalResult, NodeId, Priority, ServiceId};

/// Represents a response returned by a service
pub trait Response<const RESPONSE_SIZE: usize, N: NodeId, S: ServiceId>: Sized {
    /// Constructs a new response
    fn new(
        priority: Priority,
        service: S,
        destination: N,
        source: N,
        data: [u8; RESPONSE_SIZE],
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
    fn data(&self) -> &[u8; RESPONSE_SIZE];
}
