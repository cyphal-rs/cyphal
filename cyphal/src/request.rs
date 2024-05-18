use crate::{NodeId, Priority, Response, ServiceId};

/// Represents a resquest sent to a service
pub trait Request: Sized {
    /// Size of the request payload
    const SIZE: usize;

    /// Type representing the response returned by the service
    type Response: Response;

    /// Returns the priority level of the request
    fn priority(&self) -> Priority;

    /// Returns the Service ID the request is intended for
    fn service(&self) -> ServiceId;

    /// Returns the destination Node ID the request is intended for
    fn destination(&self) -> NodeId;

    /// Returns the Node ID where the request originates
    fn source(&self) -> NodeId;

    /// Returns the payload of the request
    fn data(&self) -> &[u8];
}
