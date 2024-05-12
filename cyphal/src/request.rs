use crate::{NodeId, Priority, Response, ServiceId};

/// Represents a resquest sent to a service
pub trait Request<const REQUEST_SIZE: usize, const RESPONSE_SIZE: usize, S: ServiceId, N: NodeId>:
    Sized
{
    /// Type representing the response returned by the service
    type Response: Response<RESPONSE_SIZE, S, N>;

    /// Returns the priority level of the request
    fn priority(&self) -> Priority;

    /// Returns the Service ID the request is intended for
    fn service(&self) -> S;

    /// Returns the destination Node ID the request is intended for
    fn destination(&self) -> N;

    /// Returns the Node ID where the request originates
    fn source(&self) -> N;

    /// Returns the payload of the request
    fn data(&self) -> &[u8; REQUEST_SIZE];
}
