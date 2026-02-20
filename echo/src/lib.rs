use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &str = "echo";
pub const SYSTEM_ADDRESS: &str = "127.0.0.1:10100";

pub const ECHO_PROCEDURE: ProcedureId = 1;

#[derive(Serializable, Deserializable)]
pub struct EchoArgs {
    pub message: String,
}
