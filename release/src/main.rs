use release::{
    AdvanceReleaseArgs, AdvanceReleaseResult, CreateReleaseArgs, CreateReleaseResult,
    GetReleaseArgs, GetReleaseResult, ListReleasesArgs, ListReleasesResult, RollbackArgs,
    RollbackResult, ADVANCE_RELEASE_PROCEDURE, CREATE_RELEASE_PROCEDURE, GET_RELEASE_PROCEDURE,
    LIST_RELEASES_PROCEDURE, ROLLBACK_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME,
};
use rpc::{server, Request, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const SCHEDULER_ADDR: &str = "127.0.0.1:10900";

struct Release {
    id: String,
    service: String,
    version: String,
    description: String,
    status: String,
    old_instances: Vec<String>,
    new_instances: Vec<String>,
    batch_progress: i32,
    total_batches: i32,
}

struct ReleaseState {
    releases: Vec<Release>,
    active: HashMap<String, String>, // service -> active release_id
    next_id: u64,
}

impl ReleaseState {
    fn new() -> Self {
        ReleaseState {
            releases: Vec::new(),
            active: HashMap::new(),
            next_id: 1,
        }
    }
}

mod handlers {
    use super::*;

    pub async fn create_release(payload: &str, state: &mut ReleaseState) -> Response {
        let args =
            CreateReleaseArgs::deserialize(payload).expect("Failed to deserialize payload");

        let id = format!("rel_{}", state.next_id);
        state.next_id += 1;

        // Get current instances from scheduler
        let svc_result =
            scheduling::get_service(SCHEDULER_ADDR, args.service.clone()).await;
        let old_instances: Vec<String> = if svc_result.instances.is_empty() {
            Vec::new()
        } else {
            svc_result
                .instances
                .split(';')
                .map(|s| s.to_string())
                .collect()
        };

        let total = old_instances.len() as i32;
        let batch_size = std::cmp::max(1, total / 10);
        let total_batches = if total > 0 {
            (total + batch_size - 1) / batch_size
        } else {
            0
        };

        let release = Release {
            id: id.clone(),
            service: args.service.clone(),
            version: args.version,
            description: args.description,
            status: "created".to_string(),
            old_instances,
            new_instances: Vec::new(),
            batch_progress: 0,
            total_batches,
        };
        state.releases.push(release);
        state.active.insert(args.service, id.clone());

        let result = CreateReleaseResult { release_id: id };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn get_release(payload: &str, state: &mut ReleaseState) -> Response {
        let args =
            GetReleaseArgs::deserialize(payload).expect("Failed to deserialize payload");

        let release = state.releases.iter().find(|r| r.id == args.release_id);
        match release {
            Some(r) => {
                let result = GetReleaseResult {
                    release_id: r.id.clone(),
                    service: r.service.clone(),
                    version: r.version.clone(),
                    description: r.description.clone(),
                    status: r.status.clone(),
                    batch_progress: r.batch_progress,
                };
                Response {
                    payload: result.serialize(),
                }
            }
            None => {
                let result = GetReleaseResult {
                    release_id: args.release_id,
                    service: String::new(),
                    version: String::new(),
                    description: String::new(),
                    status: "not_found".to_string(),
                    batch_progress: 0,
                };
                Response {
                    payload: result.serialize(),
                }
            }
        }
    }

    pub async fn list_releases(payload: &str, state: &mut ReleaseState) -> Response {
        let _args =
            ListReleasesArgs::deserialize(payload).expect("Failed to deserialize payload");

        let entries: Vec<String> = state
            .releases
            .iter()
            .map(|r| {
                format!(
                    "{}:{}:{}:{}:{}",
                    r.id, r.service, r.version, r.status, r.batch_progress
                )
            })
            .collect();

        let result = ListReleasesResult {
            releases: entries.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn advance_release(payload: &str, state: &mut ReleaseState) -> Response {
        let args =
            AdvanceReleaseArgs::deserialize(payload).expect("Failed to deserialize payload");

        let release = match state
            .releases
            .iter_mut()
            .find(|r| r.id == args.release_id)
        {
            Some(r) => r,
            None => {
                let result = AdvanceReleaseResult {
                    success: 0,
                    status: "not_found".to_string(),
                };
                return Response {
                    payload: result.serialize(),
                };
            }
        };

        if release.status != "created" && release.status != "deploying" {
            let result = AdvanceReleaseResult {
                success: 0,
                status: release.status.clone(),
            };
            return Response {
                payload: result.serialize(),
            };
        }

        release.status = "deploying".to_string();

        // Scale up: tell scheduler to add 1 more replica
        let current_total = release.old_instances.len() as i32
            + release.new_instances.len() as i32
            - release.batch_progress;
        let _ = scheduling::scale_service(
            SCHEDULER_ADDR,
            release.service.clone(),
            current_total + 1,
        )
        .await;

        // Wait for new instance to come up
        sleep(Duration::from_secs(2)).await;

        // Scale down: remove one old instance
        if !release.old_instances.is_empty() {
            let old = release.old_instances.remove(0);
            let old_id = old.split(':').next().unwrap_or("").to_string();
            if !old_id.is_empty() {
                let _ = scheduling::stop_instance(SCHEDULER_ADDR, old_id).await;
            }
        }

        release.batch_progress += 1;

        if release.old_instances.is_empty() {
            release.status = "deployed".to_string();
        }

        let result = AdvanceReleaseResult {
            success: 1,
            status: release.status.clone(),
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn rollback(payload: &str, state: &mut ReleaseState) -> Response {
        let args = RollbackArgs::deserialize(payload).expect("Failed to deserialize payload");

        if let Some(release) = state
            .releases
            .iter_mut()
            .rev()
            .find(|r| r.service == args.service && r.status == "deploying")
        {
            release.status = "rolled_back".to_string();
            let result = RollbackResult {
                success: 1,
                rolled_back_to: "previous".to_string(),
            };
            Response {
                payload: result.serialize(),
            }
        } else {
            let result = RollbackResult {
                success: 0,
                rolled_back_to: String::new(),
            };
            Response {
                payload: result.serialize(),
            }
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<ReleaseState>>) -> Response {
    let mut state = shared_state.lock().await;
    match request.procedure_id {
        CREATE_RELEASE_PROCEDURE => {
            handlers::create_release(&request.payload, &mut state).await
        }
        GET_RELEASE_PROCEDURE => handlers::get_release(&request.payload, &mut state).await,
        LIST_RELEASES_PROCEDURE => {
            handlers::list_releases(&request.payload, &mut state).await
        }
        ADVANCE_RELEASE_PROCEDURE => {
            handlers::advance_release(&request.payload, &mut state).await
        }
        ROLLBACK_PROCEDURE => handlers::rollback(&request.payload, &mut state).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ReleaseState::new()));

    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!("Release service starting on {}", addr);

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
