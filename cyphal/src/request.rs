use crate::{NodeId, Priority, Response, ServiceId};

/// Represents a resquest sent to a service
pub trait Request<const N: usize, const M: usize>: Sized {
    /// Type representing the response returned by the service
    type Response: Response<M>;

    /// Returns the priority level of the request
    fn priority(&self) -> Priority;

    /// Returns the Service ID the request is intended for
    fn service(&self) -> ServiceId;

    /// Returns the destination Node ID the request is intended for
    fn destination(&self) -> NodeId;

    /// Returns the Node ID where the request originates
    fn source(&self) -> NodeId;

    /// Returns the payload of the request
    fn data(&self) -> &[u8; N];
}
