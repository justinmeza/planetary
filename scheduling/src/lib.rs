use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "scheduling";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10900";

pub const SCHEDULE_SERVICE_PROCEDURE: ProcedureId = 401;
pub const LIST_INSTANCES_PROCEDURE: ProcedureId = 402;
pub const SCALE_SERVICE_PROCEDURE: ProcedureId = 403;
pub const STOP_INSTANCE_PROCEDURE: ProcedureId = 404;
pub const GET_SERVICE_PROCEDURE: ProcedureId = 405;

#[derive(Debug, Serializable, Deserializable)]
pub struct ScheduleServiceArgs {
    pub name: String,
    pub manifest_path: String,
    pub bin_name: String,
    pub replicas: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScheduleServiceResult {
    pub success: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListInstancesArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListInstancesResult {
    pub instances: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScaleServiceArgs {
    pub name: String,
    pub replicas: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScaleServiceResult {
    pub success: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct StopInstanceArgs {
    pub instance_id: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct StopInstanceResult {
    pub success: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetServiceArgs {
    pub name: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetServiceResult {
    pub name: String,
    pub replicas: i32,
    pub instance_count: i32,
    pub instances: String,
}

// Client helpers

use rpc::{client, Request};

pub async fn schedule_service(
    addr: &str,
    name: String,
    manifest_path: String,
    bin_name: String,
    replicas: i32,
) -> ScheduleServiceResult {
    let args = ScheduleServiceArgs {
        name,
        manifest_path,
        bin_name,
        replicas,
    };
    let request = Request {
        procedure_id: SCHEDULE_SERVICE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to schedule service");
    ScheduleServiceResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn list_instances(addr: &str) -> ListInstancesResult {
    let args = ListInstancesArgs { placeholder: 0 };
    let request = Request {
        procedure_id: LIST_INSTANCES_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to list instances");
    ListInstancesResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn scale_service(addr: &str, name: String, replicas: i32) -> ScaleServiceResult {
    let args = ScaleServiceArgs { name, replicas };
    let request = Request {
        procedure_id: SCALE_SERVICE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to scale service");
    ScaleServiceResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn stop_instance(addr: &str, instance_id: String) -> StopInstanceResult {
    let args = StopInstanceArgs { instance_id };
    let request = Request {
        procedure_id: STOP_INSTANCE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to stop instance");
    StopInstanceResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn get_service(addr: &str, name: String) -> GetServiceResult {
    let args = GetServiceArgs { name };
    let request = Request {
        procedure_id: GET_SERVICE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to get service");
    GetServiceResult::deserialize(&response.payload).expect("Failed to deserialize")
}
