use crate::{CyphalResult, Message, Request};

pub trait Transport {
    fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()>;

    fn invoque<const N: usize, const M: usize, R: Request<N, M>>(
        &mut self,
        request: &R,
    ) -> CyphalResult<R::Response>;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{CyphalResult, Message, Request, Response, TransferId, Transport};

    pub struct MockTransport {
        pub transfer_id: TransferId,
    }

    impl MockTransport {
        pub fn new() -> Self {
            MockTransport { transfer_id: 0 }
        }

        fn next_transfer_id(&mut self) -> TransferId {
            self.transfer_id += 1;

            self.transfer_id
        }
    }

    impl Transport for MockTransport {
        fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()> {
            let _ = message.payload();
            self.next_transfer_id();
            Ok(())
        }

        fn invoque<const N: usize, const M: usize, R: Request<N, M>>(
            &mut self,
            request: &R,
        ) -> CyphalResult<R::Response> {
            Ok(R::Response::new(
                request.priority(),
                request.service(),
                request.destination(),
                request.source(),
                [0; M],
            )
            .unwrap())
        }
    }
}
