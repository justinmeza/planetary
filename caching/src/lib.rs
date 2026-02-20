use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "caching";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10700";

pub const GET_PROCEDURE: ProcedureId = 1;
pub const SET_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const STATS_PROCEDURE: ProcedureId = 4;
pub const REPLICATE_SET_PROCEDURE: ProcedureId = 5;
pub const REPLICATE_DELETE_PROCEDURE: ProcedureId = 6;
pub const MODE_PROCEDURE: ProcedureId = 7;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
    pub hit: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
    pub ttl_secs: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct DeleteArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct StatsArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct StatsResult {
    pub hits: i32,
    pub misses: i32,
    pub size: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplicateSetArgs {
    pub key: String,
    pub value: String,
    pub ttl_secs: i32,
    pub version: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplicateDeleteArgs {
    pub key: String,
    pub version: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ModeArgs {
    pub mode: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ModeResult {
    pub mode: String,
}

// Client helpers
use rpc::{client, Request};

pub async fn replicate_set(
    addr: &str,
    key: String,
    value: String,
    ttl_secs: i32,
    version: i32,
) -> String {
    let args = ReplicateSetArgs {
        key,
        value,
        ttl_secs,
        version,
    };
    let request = Request {
        procedure_id: REPLICATE_SET_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => response.payload,
        Err(e) => format!("ERROR: {}", e),
    }
}

pub async fn replicate_delete_remote(addr: &str, key: String, version: i32) -> String {
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

pub async fn get_mode(addr: &str) -> ModeResult {
    let args = ModeArgs {
        mode: String::new(),
    };
    let request = Request {
        procedure_id: MODE_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => ModeResult::deserialize(&response.payload).unwrap_or(ModeResult {
            mode: "unknown".to_string(),
        }),
        Err(_) => ModeResult {
            mode: "unknown".to_string(),
        },
    }
}

pub async fn set_mode(addr: &str, mode: String) -> ModeResult {
    let args = ModeArgs { mode };
    let request = Request {
        procedure_id: MODE_PROCEDURE,
        payload: args.serialize(),
    };
    match client::send_request(addr, request).await {
        Ok(response) => ModeResult::deserialize(&response.payload).unwrap_or(ModeResult {
            mode: "unknown".to_string(),
        }),
        Err(_) => ModeResult {
            mode: "unknown".to_string(),
        },
    }
}
