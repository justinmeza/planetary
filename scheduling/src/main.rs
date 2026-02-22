use rpc::{server, Request, Response};
use scheduling::{
    GetServiceArgs, GetServiceResult, ListInstancesArgs, ListInstancesResult, ScaleServiceArgs,
    ScaleServiceResult, ScheduleServiceArgs, ScheduleServiceResult, StopInstanceArgs,
    StopInstanceResult, GET_SERVICE_PROCEDURE, LIST_INSTANCES_PROCEDURE,
    SCALE_SERVICE_PROCEDURE, SCHEDULE_SERVICE_PROCEDURE, STOP_INSTANCE_PROCEDURE,
    SYSTEM_ADDRESS, SYSTEM_NAME,
};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

struct ServiceSpec {
    name: String,
    manifest_path: String,
    bin_name: String, // empty string means default binary
    desired_replicas: i32,
}

struct Instance {
    id: String,
    service_name: String,
    port: u16,
    pid: u32,
    status: String,
}

struct SchedulerState {
    services: HashMap<String, ServiceSpec>,
    instances: Vec<Instance>,
    next_port: u16,
    next_id: u64,
    base_dir: String,
}

impl SchedulerState {
    fn new(base_dir: String) -> Self {
        SchedulerState {
            services: HashMap::new(),
            instances: Vec::new(),
            next_port: 11200,
            next_id: 1,
            base_dir,
        }
    }

    fn allocate_port(&mut self) -> u16 {
        let port = self.next_port;
        self.next_port += 1;
        port
    }

    fn spawn_instance(&mut self, spec: &ServiceSpec, port: u16) -> Option<Instance> {
        let manifest = format!("{}/{}", self.base_dir, spec.manifest_path);
        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("--manifest-path").arg(&manifest);

        if !spec.bin_name.is_empty() {
            cmd.arg("--bin").arg(&spec.bin_name);
        }

        cmd.env("PORT", port.to_string());
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        match cmd.spawn() {
            Ok(child) => {
                let id = format!("{}_{}", spec.name, self.next_id);
                self.next_id += 1;
                let pid = child.id();
                println!(
                    "Spawned {} instance {} on port {} (pid {})",
                    spec.name, id, port, pid
                );
                Some(Instance {
                    id,
                    service_name: spec.name.clone(),
                    port,
                    pid,
                    status: "starting".to_string(),
                })
            }
            Err(e) => {
                eprintln!("Failed to spawn {}: {}", spec.name, e);
                None
            }
        }
    }

    fn reconcile(&mut self, name: &str) {
        let spec = match self.services.get(name) {
            Some(s) => ServiceSpec {
                name: s.name.clone(),
                manifest_path: s.manifest_path.clone(),
                bin_name: s.bin_name.clone(),
                desired_replicas: s.desired_replicas,
            },
            None => return,
        };

        let current_count = self
            .instances
            .iter()
            .filter(|i| i.service_name == name && i.status != "stopped")
            .count() as i32;

        if current_count < spec.desired_replicas {
            let needed = spec.desired_replicas - current_count;
            for _ in 0..needed {
                let port = self.allocate_port();
                if let Some(instance) = self.spawn_instance(&spec, port) {
                    self.instances.push(instance);
                }
            }
        }
    }
}

// Default fleet configuration with well-known ports for single-replica services
struct FleetEntry {
    name: &'static str,
    manifest_path: &'static str,
    bin_name: &'static str,
    replicas: i32,
    base_port: u16,
}

const DEFAULT_FLEET: &[FleetEntry] = &[
    FleetEntry {
        name: "security",
        manifest_path: "security/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 11100,
    },
    FleetEntry {
        name: "monitoring",
        manifest_path: "monitoring/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 10800,
    },
    FleetEntry {
        name: "configuration",
        manifest_path: "configuration/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 10500,
    },
    FleetEntry {
        name: "storage",
        manifest_path: "storage/Cargo.toml",
        bin_name: "",
        replicas: 3,
        base_port: 10600,
    },
    FleetEntry {
        name: "caching",
        manifest_path: "caching/Cargo.toml",
        bin_name: "",
        replicas: 3,
        base_port: 10700,
    },
    FleetEntry {
        name: "routing",
        manifest_path: "routing/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 10300,
    },
    FleetEntry {
        name: "echo",
        manifest_path: "echo/Cargo.toml",
        bin_name: "server_v1",
        replicas: 3,
        base_port: 10100,
    },
    FleetEntry {
        name: "release",
        manifest_path: "release/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 11000,
    },
    FleetEntry {
        name: "tailer",
        manifest_path: "tailer/Cargo.toml",
        bin_name: "",
        replicas: 1,
        base_port: 10400,
    },
    FleetEntry {
        name: "frontend",
        manifest_path: "frontend/Cargo.toml",
        bin_name: "",
        replicas: 2,
        base_port: 8081,
    },
];

fn get_replicas(name: &str, default: i32) -> i32 {
    let key = format!("{}_REPLICAS", name.to_uppercase());
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn bootstrap_fleet(state: &mut SchedulerState) {
    for entry in DEFAULT_FLEET {
        let replicas = get_replicas(entry.name, entry.replicas);
        let spec = ServiceSpec {
            name: entry.name.to_string(),
            manifest_path: entry.manifest_path.to_string(),
            bin_name: entry.bin_name.to_string(),
            desired_replicas: replicas,
        };

        // Spawn instances with well-known ports
        for i in 0..replicas {
            let port = entry.base_port + i as u16;
            if let Some(instance) = state.spawn_instance(&spec, port) {
                state.instances.push(instance);
            }
        }

        state.services.insert(entry.name.to_string(), spec);
    }
}

mod handlers {
    use super::*;

    pub async fn schedule_service(payload: &str, state: &mut SchedulerState) -> Response {
        let args = ScheduleServiceArgs::deserialize(payload)
            .expect("Failed to deserialize payload");

        let spec = ServiceSpec {
            name: args.name.clone(),
            manifest_path: args.manifest_path,
            bin_name: args.bin_name,
            desired_replicas: args.replicas,
        };
        state.services.insert(args.name.clone(), spec);
        state.reconcile(&args.name);

        let result = ScheduleServiceResult { success: 1 };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn list_instances(payload: &str, state: &mut SchedulerState) -> Response {
        let _args = ListInstancesArgs::deserialize(payload)
            .expect("Failed to deserialize payload");

        let entries: Vec<String> = state
            .instances
            .iter()
            .filter(|i| i.status != "stopped")
            .map(|i| {
                format!(
                    "{}:{}:127.0.0.1:{}:{}:{}",
                    i.id, i.service_name, i.port, i.pid, i.status
                )
            })
            .collect();

        let result = ListInstancesResult {
            instances: entries.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn scale_service(payload: &str, state: &mut SchedulerState) -> Response {
        let args =
            ScaleServiceArgs::deserialize(payload).expect("Failed to deserialize payload");

        if let Some(spec) = state.services.get_mut(&args.name) {
            spec.desired_replicas = args.replicas;
        }
        state.reconcile(&args.name);

        let result = ScaleServiceResult { success: 1 };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn stop_instance(payload: &str, state: &mut SchedulerState) -> Response {
        let args =
            StopInstanceArgs::deserialize(payload).expect("Failed to deserialize payload");

        let mut success = 0;
        for instance in state.instances.iter_mut() {
            if instance.id == args.instance_id {
                // Kill the process
                libc_kill(instance.pid as i32);
                instance.status = "stopped".to_string();
                success = 1;
                println!("Stopped instance {} (pid {})", instance.id, instance.pid);
                break;
            }
        }

        let result = StopInstanceResult { success };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn get_service(payload: &str, state: &mut SchedulerState) -> Response {
        let args =
            GetServiceArgs::deserialize(payload).expect("Failed to deserialize payload");

        let replicas = state
            .services
            .get(&args.name)
            .map(|s| s.desired_replicas)
            .unwrap_or(0);

        let running: Vec<&Instance> = state
            .instances
            .iter()
            .filter(|i| i.service_name == args.name && i.status != "stopped")
            .collect();

        let instance_strs: Vec<String> = running
            .iter()
            .map(|i| format!("{}:127.0.0.1:{}:{}", i.id, i.port, i.status))
            .collect();

        let result = GetServiceResult {
            name: args.name,
            replicas,
            instance_count: running.len() as i32,
            instances: instance_strs.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }

    // Send SIGKILL via libc
    fn libc_kill(pid: i32) {
        unsafe {
            #[cfg(unix)]
            {
                extern "C" {
                    fn kill(pid: i32, sig: i32) -> i32;
                }
                kill(pid, 9); // SIGKILL
            }
        }
    }
}

async fn request_handler(
    request: Request,
    shared_state: Arc<Mutex<SchedulerState>>,
) -> Response {
    let mut state = shared_state.lock().await;
    match request.procedure_id {
        SCHEDULE_SERVICE_PROCEDURE => {
            handlers::schedule_service(&request.payload, &mut state).await
        }
        LIST_INSTANCES_PROCEDURE => {
            handlers::list_instances(&request.payload, &mut state).await
        }
        SCALE_SERVICE_PROCEDURE => {
            handlers::scale_service(&request.payload, &mut state).await
        }
        STOP_INSTANCE_PROCEDURE => {
            handlers::stop_instance(&request.payload, &mut state).await
        }
        GET_SERVICE_PROCEDURE => handlers::get_service(&request.payload, &mut state).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

async fn health_check_loop(shared_state: Arc<Mutex<SchedulerState>>) {
    loop {
        sleep(Duration::from_secs(5)).await;
        let mut state = shared_state.lock().await;
        for instance in state.instances.iter_mut() {
            if instance.status == "stopped" {
                continue;
            }
            // TCP probe to check health
            let addr = format!("127.0.0.1:{}", instance.port);
            match tokio::net::TcpStream::connect(&addr).await {
                Ok(_) => {
                    if instance.status != "healthy" {
                        println!("Instance {} is now healthy", instance.id);
                        instance.status = "healthy".to_string();
                    }
                }
                Err(_) => {
                    if instance.status == "healthy" {
                        println!("Instance {} is now unhealthy", instance.id);
                        instance.status = "unhealthy".to_string();
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let base_dir = std::env::var("BASE_DIR").unwrap_or_else(|_| {
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string()
    });

    let state = Arc::new(Mutex::new(SchedulerState::new(base_dir)));

    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    // Bootstrap the fleet
    {
        let mut s = state.lock().await;
        bootstrap_fleet(&mut s);
    }

    println!("Scheduling service starting on {}", addr);
    println!("Fleet bootstrap complete");

    // Start health check loop
    let health_state = Arc::clone(&state);
    tokio::spawn(async move {
        health_check_loop(health_state).await;
    });

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
