use crate::get_info::{GetInfoRequest, GetInfoResponse, GET_INFO_PORT_ID};
use cyphal::{
    CyphalResult, NodeId, Priority, Request, Response, Router as CyphalRouter, ServiceId,
};

pub struct Router {}

impl CyphalRouter for Router {
    async fn process_request(
        &self,
        priority: Priority,
        service: ServiceId,
        source: NodeId,
        destination: NodeId,
        data: &[u8],
    ) -> CyphalResult<Option<impl Response>> {
        if service == GET_INFO_PORT_ID {
            let _request = GetInfoRequest::new_raw(priority, service, source, destination, data)?;

            let response = GetInfoResponse::new(destination, source)?;
            return Ok(Some(response));
        }

        Ok(None)
    }
}
