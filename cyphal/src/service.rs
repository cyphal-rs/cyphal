use crate::{CyphalResult, NodeId, Priority, ServiceId};

pub trait Request<const N: usize, const M: usize>: Sized {
    type Payload: Sized;

    type Response: Response<M>;

    fn priority(&self) -> Priority;

    fn service(&self) -> ServiceId;

    fn destination(&self) -> NodeId;

    fn source(&self) -> NodeId;

    fn payload(&self) -> &[u8];
}

pub trait Response<const N: usize>: Sized {
    type Payload: Sized;

    fn new(
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
        payload: [u8; N],
    ) -> CyphalResult<Self>;

    fn priority(&self) -> Priority;

    fn service(&self) -> ServiceId;

    fn destination(&self) -> NodeId;

    fn source(&self) -> NodeId;

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
