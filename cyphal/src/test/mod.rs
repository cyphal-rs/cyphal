mod message;
pub use message::TestMessage;

mod request;
pub use request::TestRequest;

mod response;
pub use response::TestResponse;

mod router;
pub use router::TestRouter;

mod transfer_id;
pub use transfer_id::TestTransferId;

mod transport;
pub use transport::TestTransport;

pub const TEST_MESSAGE_SIZE: usize = 78;
pub const TEST_REQUEST_SIZE: usize = 15;
pub const TEST_RESPONSE_SIZE: usize = 86;
pub const TEST_MAX_TRANSFER_ID: u8 = 12;
