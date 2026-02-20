use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "monitoring";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10800";

pub const REPORT_PROCEDURE: ProcedureId = 1;
pub const HEARTBEAT_PROCEDURE: ProcedureId = 2;
pub const QUERY_PROCEDURE: ProcedureId = 3;
pub const HEALTH_PROCEDURE: ProcedureId = 4;

#[derive(Debug, Serializable, Deserializable)]
pub struct ReportArgs {
    pub service: String,
    pub metric: String,
    pub value: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HeartbeatArgs {
    pub service: String,
    pub status: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryArgs {
    pub service: String,
    pub metric: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryResult {
    pub values: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HealthArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HealthResult {
    pub services: String,
}
