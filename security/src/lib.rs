use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "security";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:11100";

pub const CREATE_TOKEN_PROCEDURE: ProcedureId = 601;
pub const VALIDATE_TOKEN_PROCEDURE: ProcedureId = 602;
pub const REVOKE_TOKEN_PROCEDURE: ProcedureId = 603;
pub const LIST_TOKENS_PROCEDURE: ProcedureId = 604;

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateTokenArgs {
    pub name: String,
    pub permissions: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateTokenResult {
    pub token: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ValidateTokenArgs {
    pub token: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ValidateTokenResult {
    pub valid: i32,
    pub name: String,
    pub permissions: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RevokeTokenArgs {
    pub token: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RevokeTokenResult {
    pub success: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListTokensArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListTokensResult {
    pub tokens: String,
}

// Client helpers

use rpc::{client, Request};

pub async fn create_token(addr: &str, name: String, permissions: String) -> CreateTokenResult {
    let args = CreateTokenArgs { name, permissions };
    let request = Request {
        procedure_id: CREATE_TOKEN_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request).await.expect("Failed to create token");
    CreateTokenResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn validate_token(addr: &str, token: String) -> ValidateTokenResult {
    let args = ValidateTokenArgs { token };
    let request = Request {
        procedure_id: VALIDATE_TOKEN_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request).await.expect("Failed to validate token");
    ValidateTokenResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn revoke_token(addr: &str, token: String) -> RevokeTokenResult {
    let args = RevokeTokenArgs { token };
    let request = Request {
        procedure_id: REVOKE_TOKEN_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request).await.expect("Failed to revoke token");
    RevokeTokenResult::deserialize(&response.payload).expect("Failed to deserialize")
}

pub async fn list_tokens(addr: &str) -> ListTokensResult {
    let args = ListTokensArgs { placeholder: 0 };
    let request = Request {
        procedure_id: LIST_TOKENS_PROCEDURE,
        payload: args.serialize(),
    };
    let response = client::send_request(addr, request).await.expect("Failed to list tokens");
    ListTokensResult::deserialize(&response.payload).expect("Failed to deserialize")
}
