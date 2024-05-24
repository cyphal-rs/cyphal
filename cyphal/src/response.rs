use crate::{CyphalResult, NodeId, Priority, ServiceId};

/// Represents a response returned by a service
pub trait Response: Sized {
    /// Size of the response payload
    const SIZE: usize;

    /// Constructs a new response
    fn new_raw(
        priority: Priority,
        service: ServiceId,
        source: NodeId,
        destination: NodeId,
        data: &[u8],
    ) -> CyphalResult<Self>;

    /// Returns the priority level of the response
    fn priority(&self) -> Priority;

    /// Returns the Service ID where the response originates
    fn service(&self) -> ServiceId;

    /// Returns the destination Node ID where the response originates
    fn destination(&self) -> NodeId;

    /// Returns the Node ID that requested the response
    fn source(&self) -> NodeId;

    /// Returns the payload of the response
    fn data(&self) -> &[u8];
}
