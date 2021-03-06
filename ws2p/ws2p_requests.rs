extern crate duniter_crypto;
extern crate duniter_network;
extern crate serde;
extern crate serde_json;

use duniter_network::NetworkRequest;

pub fn network_request_to_json(request: &NetworkRequest) -> serde_json::Value {
    let (request_id, request_type, request_params) = match *request {
        NetworkRequest::GetCurrent(ref req_full_id, _receiver) => {
            (req_full_id.1, "CURRENT", json!({}))
        }
        NetworkRequest::GetBlocks(ref req_full_id, _receiver, count, from_mumber) => (
            req_full_id.1,
            "BLOCKS_CHUNK",
            json!({
                    "count": count,
                    "fromNumber": from_mumber
                }),
        ),
        NetworkRequest::GetRequirementsPending(ref req_full_id, _receiver, min_cert) => (
            req_full_id.1,
            "WOT_REQUIREMENTS_OF_PENDING",
            json!({ "minCert": min_cert }),
        ),
        NetworkRequest::GetConsensus(_) => {
            panic!("GetConsensus() request must be not convert to json !");
        }
        NetworkRequest::GetHeadsCache(_) => {
            panic!("GetHeadsCache() request must be not convert to json !");
        }
        NetworkRequest::GetEndpoints(_) => {
            panic!("GetEndpoints() request must be not convert to json !");
        }
    };

    json!({
            "reqId": request_id,
            "body" : {
                "name": request_type,
                "params": request_params
            }
        })
}
