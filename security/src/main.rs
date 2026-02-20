use rpc::{server, Request, Response};
use security::{
    CreateTokenArgs, CreateTokenResult, ListTokensArgs, ListTokensResult, RevokeTokenArgs,
    RevokeTokenResult, ValidateTokenArgs, ValidateTokenResult, CREATE_TOKEN_PROCEDURE,
    LIST_TOKENS_PROCEDURE, REVOKE_TOKEN_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME,
    VALIDATE_TOKEN_PROCEDURE,
};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

struct TokenEntry {
    name: String,
    token: String,
    permissions: String,
    created_at: u64,
}

struct SecurityState {
    tokens: HashMap<String, TokenEntry>,
    rng_state: u64,
}

impl SecurityState {
    fn new() -> Self {
        // Seed from system time
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        SecurityState {
            tokens: HashMap::new(),
            rng_state: seed | 1, // Ensure non-zero
        }
    }

    fn xorshift64(&mut self) -> u64 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        x
    }

    fn generate_token(&mut self) -> String {
        let a = self.xorshift64();
        let b = self.xorshift64();
        format!("{:016x}{:016x}", a, b)
    }
}

mod handlers {
    use super::*;

    pub async fn create_token(payload: &str, state: &mut SecurityState) -> Response {
        let args =
            CreateTokenArgs::deserialize(payload).expect("Failed to deserialize payload");
        let token = state.generate_token();
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let entry = TokenEntry {
            name: args.name,
            token: token.clone(),
            permissions: args.permissions,
            created_at,
        };
        state.tokens.insert(token.clone(), entry);
        let result = CreateTokenResult { token };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn validate_token(payload: &str, state: &mut SecurityState) -> Response {
        let args =
            ValidateTokenArgs::deserialize(payload).expect("Failed to deserialize payload");
        match state.tokens.get(&args.token) {
            Some(entry) => {
                let result = ValidateTokenResult {
                    valid: 1,
                    name: entry.name.clone(),
                    permissions: entry.permissions.clone(),
                };
                Response {
                    payload: result.serialize(),
                }
            }
            None => {
                let result = ValidateTokenResult {
                    valid: 0,
                    name: String::new(),
                    permissions: String::new(),
                };
                Response {
                    payload: result.serialize(),
                }
            }
        }
    }

    pub async fn revoke_token(payload: &str, state: &mut SecurityState) -> Response {
        let args =
            RevokeTokenArgs::deserialize(payload).expect("Failed to deserialize payload");
        let removed = state.tokens.remove(&args.token).is_some();
        let result = RevokeTokenResult {
            success: if removed { 1 } else { 0 },
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn list_tokens(payload: &str, state: &mut SecurityState) -> Response {
        let _args =
            ListTokensArgs::deserialize(payload).expect("Failed to deserialize payload");
        let tokens: Vec<String> = state
            .tokens
            .values()
            .map(|entry| {
                format!(
                    "{}:{}:{}:{}",
                    entry.name, entry.token, entry.permissions, entry.created_at
                )
            })
            .collect();
        let result = ListTokensResult {
            tokens: tokens.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<SecurityState>>) -> Response {
    let mut state = shared_state.lock().await;
    match request.procedure_id {
        CREATE_TOKEN_PROCEDURE => handlers::create_token(&request.payload, &mut state).await,
        VALIDATE_TOKEN_PROCEDURE => {
            handlers::validate_token(&request.payload, &mut state).await
        }
        REVOKE_TOKEN_PROCEDURE => handlers::revoke_token(&request.payload, &mut state).await,
        LIST_TOKENS_PROCEDURE => handlers::list_tokens(&request.payload, &mut state).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let mut initial_state = SecurityState::new();
    if let Ok(token) = std::env::var("ADMIN_TOKEN") {
        if !token.is_empty() {
            let created_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            initial_state.tokens.insert(
                token.clone(),
                TokenEntry {
                    name: "admin".to_string(),
                    token,
                    permissions: "admin".to_string(),
                    created_at,
                },
            );
            println!("Admin token seeded from ADMIN_TOKEN env var");
        }
    }
    let state = Arc::new(Mutex::new(initial_state));

    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!("Security service starting on {}", addr);

    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        state,
    )
    .await
    .expect("Server crashed");
}
