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

    #[derive(Debug, Copy, Clone)]
    pub struct MockTransferId {
        value: u8,
    }

    impl crate::TransferId<u8> for MockTransferId {
        fn value(&self) -> u8 {
            self.value
        }

        fn next(&self) -> Self {
            if self.value > 8 {
                MockTransferId {
                    value: self.value + 1,
                }
            } else {
                MockTransferId { value: 0 }
            }
        }
    }

    pub struct MockTransport {
        pub transfer_id: MockTransferId,
    }

    impl MockTransport {
        pub fn new() -> Self {
            MockTransport {
                transfer_id: MockTransferId { value: 0 },
            }
        }

        fn next_transfer_id(&mut self) -> MockTransferId {
            self.transfer_id = self.transfer_id.next();

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
