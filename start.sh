#!/bin/bash
set -e

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"
export CARGO_TARGET_DIR="$BASE_DIR/target"
PIDS=()

kill_tree() {
    local pid=$1
    local sig=${2:-TERM}
    for child in $(pgrep -P "$pid" 2>/dev/null); do
        kill_tree "$child" "$sig"
    done
    kill "-$sig" "$pid" 2>/dev/null
}

PORTS=(8080 10100 10200 10300 10500 10600 10601 10602 10700 10701 10702 10800 10900 11000 11100)

kill_port_holders() {
    for port in "${PORTS[@]}"; do
        pids=$(lsof -ti tcp:"$port" 2>/dev/null) || true
        if [ -n "$pids" ]; then
            echo "$pids" | xargs kill "$@" 2>/dev/null || true
        fi
    done
}

cleanup() {
    echo ""
    echo "Stopping all services..."
    for pid in "${PIDS[@]}"; do
        kill_tree "$pid" TERM
    done
    kill_port_holders
    sleep 2
    for pid in "${PIDS[@]}"; do
        kill_tree "$pid" 9
    done
    kill_port_holders -9
    wait 2>/dev/null
    echo "All services stopped."
}

trap cleanup EXIT INT TERM

start_service() {
    local name="$1"
    local dir="$2"
    local bin_flag="$3"

    if [ -n "$bin_flag" ]; then
        cargo run --bin "$bin_flag" --manifest-path "$BASE_DIR/$dir/Cargo.toml" >/dev/null 2>&1 &
    else
        cargo run --manifest-path "$BASE_DIR/$dir/Cargo.toml" >/dev/null 2>&1 &
    fi
    PIDS+=($!)
    echo "  Started $name (pid $!)"
}

wait_for_port() {
    local name="$1"
    local port="$2"
    local timeout=30
    local elapsed=0

    while ! nc -z 127.0.0.1 "$port" 2>/dev/null; do
        sleep 0.2
        elapsed=$((elapsed + 1))
        if [ "$elapsed" -ge $((timeout * 5)) ]; then
            echo "  WARNING: $name not yet ready on port $port (continuing anyway)"
            return
        fi
    done
    echo "  $name ready on port $port"
}

echo "=== Starting Planetary Computer ==="
echo ""

# Phase 0: Kill stale processes from previous runs
kill_port_holders
sleep 0.5
kill_port_holders -9

# Generate admin token for security service
export ADMIN_TOKEN=$(openssl rand -hex 16)
echo "  Admin token: $ADMIN_TOKEN"
echo ""

# Phase 1: Build everything
echo "Building all services..."
for dir in normalization rpc discovery scheduling security monitoring \
           configuration storage caching routing echo release frontend loadbalancer; do
    if [ -f "$BASE_DIR/$dir/Cargo.toml" ]; then
        cargo build --manifest-path "$BASE_DIR/$dir/Cargo.toml" 2>/dev/null
    fi
done
echo "  Build complete"

# Phase 2: Start infrastructure (discovery must be first)
echo ""
echo "Starting infrastructure..."
start_service discovery discovery discovery
wait_for_port discovery 10200

# Phase 3: Start scheduler (it bootstraps the rest of the fleet)
echo ""
echo "Starting scheduler..."
BASE_DIR="$BASE_DIR" start_service scheduler scheduling
wait_for_port scheduler 10900

# Phase 4: Wait for fleet to come up
echo ""
echo "Waiting for fleet..."
wait_for_port security 11100
wait_for_port monitoring 10800
wait_for_port configuration 10500
wait_for_port storage 10600
wait_for_port storage-2 10601
wait_for_port storage-3 10602
wait_for_port caching 10700
wait_for_port caching-2 10701
wait_for_port caching-3 10702
wait_for_port routing 10300
wait_for_port release 11000
wait_for_port echo 10100

# Phase 5: Start HTTP load balancer
echo ""
echo "Starting load balancer..."
start_service loadbalancer loadbalancer
wait_for_port loadbalancer 8080

echo ""
echo "=== All services running ==="
echo "Open http://127.0.0.1:8080 in your browser"
echo "Press Ctrl-C to stop everything"
echo ""

wait
