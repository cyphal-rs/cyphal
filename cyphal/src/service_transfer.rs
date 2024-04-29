// // #[cfg(feature = "crc")]
// // use crate::crc::crc32c;

// use crate::{NodeId, Priority, TransferId};

// pub trait ServiceTransfer {
//     fn service(&self) -> NodeId;

//     fn source(&self) -> NodeId;

//     fn priority(&self) -> Priority;

//     fn id(&self) -> TransferId;

//     fn payload(&self) -> &[u8];
// }

// #[cfg(test)]
// mod test {
//     use crate::{NodeId, Priority, TransferId};

//     pub struct MockServiceTransfer {
//         priority: Priority,
//         service: NodeId,
//         id: TransferId,
//         source: NodeId,
//         payload: [u8; 1],
//     }

//     impl MockServiceTransfer {
//         pub fn new(
//             priority: Priority,
//             service: NodeId,
//             source: NodeId,
//             payload: &[u8; 1],
//         ) -> MockServiceTransfer {
//             MockServiceTransfer {
//                 priority,
//                 service,
//                 id: 0,
//                 source,
//                 payload: *payload,
//             }
//         }

//         pub fn service(&self) -> NodeId {
//             self.service
//         }

//         pub fn source(&self) -> NodeId {
//             self.source
//         }

//         pub fn priority(&self) -> Priority {
//             self.priority
//         }

//         pub fn id(&self) -> TransferId {
//             self.id
//         }

//         // #[cfg(feature = "crc")]
//         // fn crc(&self) -> u32 {
//         //     crc32c(&self.payload)
//         // }

//         pub fn payload(&self) -> &[u8] {
//             &self.payload
//         }
//     }
// }
