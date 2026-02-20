use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "release";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:11000";

pub const CREATE_RELEASE_PROCEDURE: ProcedureId = 501;
pub const GET_RELEASE_PROCEDURE: ProcedureId = 502;
pub const LIST_RELEASES_PROCEDURE: ProcedureId = 503;
pub const ADVANCE_RELEASE_PROCEDURE: ProcedureId = 504;
pub const ROLLBACK_PROCEDURE: ProcedureId = 505;

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateReleaseArgs {
    pub service: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateReleaseResult {
    pub release_id: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetReleaseArgs {
    pub release_id: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetReleaseResult {
    pub release_id: String,
    pub service: String,
    pub version: String,
    pub description: String,
    pub status: String,
    pub batch_progress: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListReleasesArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListReleasesResult {
    pub releases: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct AdvanceReleaseArgs {
    pub release_id: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct AdvanceReleaseResult {
    pub success: i32,
    pub status: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RollbackArgs {
    pub service: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RollbackResult {
    pub success: i32,
    pub rolled_back_to: String,
}

// Client helpers

use rpc::{client, Request};

pub async fn create_release(
    addr: &str,
    service: String,
    version: String,
    description: String,
) -> CreateReleaseResult {
    let args = CreateReleaseArgs {
        service,
        version,
        description,
    };
    let request = Request {
        procedure_id: CREATE_RELEASE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to create release");
    CreateReleaseResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn get_release(addr: &str, release_id: String) -> GetReleaseResult {
    let args = GetReleaseArgs { release_id };
    let request = Request {
        procedure_id: GET_RELEASE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to get release");
    GetReleaseResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn list_releases(addr: &str) -> ListReleasesResult {
    let args = ListReleasesArgs { placeholder: 0 };
    let request = Request {
        procedure_id: LIST_RELEASES_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to list releases");
    ListReleasesResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn advance_release(addr: &str, release_id: String) -> AdvanceReleaseResult {
    let args = AdvanceReleaseArgs { release_id };
    let request = Request {
        procedure_id: ADVANCE_RELEASE_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to advance release");
    AdvanceReleaseResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn rollback(addr: &str, service: String) -> RollbackResult {
    let args = RollbackArgs { service };
    let request = Request {
        procedure_id: ROLLBACK_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request)
        .await
        .expect("Failed to rollback");
    RollbackResult::deserialize(&response.payload).expect("Failed to deserialize")
}
