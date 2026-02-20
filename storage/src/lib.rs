use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "storage";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10600";

pub const GET_PROCEDURE: ProcedureId = 1;
pub const PUT_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const SCAN_PROCEDURE: ProcedureId = 4;
pub const REPLICATE_PUT_PROCEDURE: ProcedureId = 5;
pub const REPLICATE_DELETE_PROCEDURE: ProcedureId = 6;
pub const GET_PEERS_PROCEDURE: ProcedureId = 7;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
    pub found: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct PutArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct DeleteArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScanArgs {
    pub prefix: String,
    pub limit: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScanResult {
    pub entries: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplicatePutArgs {
    pub key: String,
    pub value: String,
    pub version: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplicateDeleteArgs {
    pub key: String,
    pub version: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetPeersArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetPeersResult {
    pub peer_count: i32,
    pub quorum_w: i32,
    pub quorum_r: i32,
}

// Client helpers for replication
use rpc::{client, Request};

pub async fn replicate_put(addr: &str, key: String, value: String, version: i32) -> String {
    let args = ReplicatePutArgs { key, value, version };
    let request = Request {
        procedure_id: REPLICATE_PUT_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => response.payload,
        Err(e) => format!("ERROR: {}", e),
    }
}

pub async fn replicate_delete(addr: &str, key: String, version: i32) -> String {
    let args = ReplicateDeleteArgs { key, version };
    let request = Request {
        procedure_id: REPLICATE_DELETE_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => response.payload,
        Err(e) => format!("ERROR: {}", e),
    }
}

pub async fn get_peers(addr: &str) -> GetPeersResult {
    let args = GetPeersArgs { placeholder: 0 };
    let request = Request {
        procedure_id: GET_PEERS_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => {
            GetPeersResult::deserialize(&response.payload).unwrap_or(GetPeersResult {
                peer_count: 0,
                quorum_w: 0,
                quorum_r: 0,
            })
        }
        Err(_) => GetPeersResult {
            peer_count: 0,
            quorum_w: 0,
            quorum_r: 0,
        },
    }
}

// Remote get for quorum reads
pub async fn remote_get(addr: &str, key: String) -> GetResult {
    let args = GetArgs { key };
    let request = Request {
        procedure_id: GET_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => GetResult::deserialize(&response.payload).unwrap_or(GetResult {
            value: String::new(),
            found: 0,
        }),
        Err(_) => GetResult {
            value: String::new(),
            found: 0,
        },
    }
}
