use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "configuration";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10500";

pub const GET_PROCEDURE: ProcedureId = 1;
pub const SET_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const LIST_PROCEDURE: ProcedureId = 4;
pub const WATCH_PROCEDURE: ProcedureId = 5;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct DeleteArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListArgs {
    pub prefix: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListResult {
    pub keys: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct WatchArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct WatchEvent {
    pub key: String,
    pub value: String,
}
