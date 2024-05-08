use crate::{CyphalResult, NodeId, Priority, ServiceId};

/// Represents a resquest sent to a service
pub trait Request<const N: usize, const M: usize>: Sized {
    /// Type representing the payload in the request
    type Payload: Sized;

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
    fn payload(&self) -> &[u8];
}

/// Represents a response returned by a service
pub trait Response<const N: usize>: Sized {
    /// Type representing the payload in the response
    type Payload: Sized;

    /// Constructs a new response
    fn new(
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
        payload: [u8; N],
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
    fn payload(&self) -> &[u8];
}

#[cfg(test)]
mod test {
    use crate::{
        transport::test::MockTransport, CyphalResult, NodeId, Priority, Request, Response,
        ServiceId, Transport,
    };

    pub struct MockRequest {
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
        payload: [u8; 1],
    }

    impl MockRequest {
        pub fn new(
            priority: Priority,
            service: ServiceId,
            destination: NodeId,
            source: NodeId,
            payload: [u8; 1],
        ) -> CyphalResult<Self> {
            Ok(Self {
                priority,
                service,
                destination,
                source,
                payload,
            })
        }
    }

    impl Request<1, 2> for MockRequest {
        type Payload = [u8; 1];

        type Response = MockResponse;

        fn priority(&self) -> Priority {
            self.priority
        }

        fn service(&self) -> ServiceId {
            self.service
        }

        fn destination(&self) -> NodeId {
            self.destination
        }

        fn source(&self) -> NodeId {
            self.source
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    pub struct MockResponse {
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
        payload: [u8; 2],
    }

    impl Response<2> for MockResponse {
        type Payload = [u8; 2];

        fn new(
            priority: Priority,
            service: ServiceId,
            destination: NodeId,
            source: NodeId,
            payload: [u8; 2],
        ) -> CyphalResult<Self> {
            Ok(Self {
                priority,
                service,
                destination,
                source,
                payload,
            })
        }

        fn priority(&self) -> Priority {
            self.priority
        }

        fn service(&self) -> ServiceId {
            self.service
        }

        fn destination(&self) -> NodeId {
            self.destination
        }

        fn source(&self) -> NodeId {
            self.source
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    #[test]
    fn new() {
        let request = MockRequest::new(Priority::Nominal, 1, 2, 3, [0]).unwrap();
        assert_eq!(request.payload().len(), 1);

        let mut transport = MockTransport::new();
        let _ = transport.invoque(&request).unwrap();
    }
}
