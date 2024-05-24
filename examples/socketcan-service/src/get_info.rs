use cyphal::{CyphalError, CyphalResult, NodeId, Priority, Request, Response, ServiceId};

pub const GET_INFO_PORT_ID: ServiceId = 430;
const GET_INFO_REQUEST_SIZE: usize = 0;
const GET_INFO_RESPONSE_SIZE: usize = 2;

pub struct GetInfoRequest {
    destination: NodeId,
    source: NodeId,
    data: [u8; GET_INFO_REQUEST_SIZE],
}

// impl GetInfoRequest {
//     pub fn new(
//         destination: NodeId,
//         source: NodeId,
//         data: [u8; GET_INFO_REQUEST_SIZE],
//     ) -> CyphalResult<Self> {
//         Ok(Self {
//             destination,
//             source,
//             data,
//         })
//     }
// }

impl Request for GetInfoRequest {
    const SIZE: usize = GET_INFO_REQUEST_SIZE;

    type Response = GetInfoResponse;

    fn new_raw(
        _priority: Priority,
        _service: ServiceId,
        _source: NodeId,
        _destination: NodeId,
        _data: &[u8],
    ) -> CyphalResult<Self> {
        todo!()
    }

    fn priority(&self) -> Priority {
        Priority::Nominal
    }

    fn service(&self) -> ServiceId {
        GET_INFO_PORT_ID
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn source(&self) -> NodeId {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

pub struct GetInfoResponse {
    destination: NodeId,
    source: NodeId,
    data: [u8; GET_INFO_RESPONSE_SIZE],
}

impl GetInfoResponse {
    pub fn new(_source: NodeId, _destination: NodeId) -> CyphalResult<Self> {
        todo!()
    }
}

impl Response for GetInfoResponse {
    const SIZE: usize = GET_INFO_RESPONSE_SIZE;

    fn new_raw(
        priority: Priority,
        service: ServiceId,
        source: NodeId,
        destination: NodeId,
        data: &[u8],
    ) -> CyphalResult<Self> {
        if priority != Priority::Nominal || service != GET_INFO_PORT_ID || data.len() != Self::SIZE
        {
            return Err(CyphalError::OutOfRange);
        }

        let mut d: [u8; Self::SIZE] = [0; Self::SIZE];
        d.copy_from_slice(data);

        Ok(Self {
            destination,
            source,
            data: d,
        })
    }

    fn priority(&self) -> Priority {
        Priority::Nominal
    }

    fn service(&self) -> ServiceId {
        GET_INFO_PORT_ID
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn source(&self) -> NodeId {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
