#!/bin/sh

set -o errexit

SINGLE_MODE=false
BUILD_MODE="debug"
LOGLEVEL="info"

for arg in "$@"; do
  case $arg in
    --single)
      SINGLE_MODE=true
      echo "Running in single node mode"
      ;;
    --release)
      BUILD_MODE="release"
      echo "Release Build mode"
      ;;
    --loglevel=*)
      LOGLEVEL="${arg#*=}"
      echo "Log level set to $LOGLEVEL"
      ;;
  esac
done

TARGET=$1

# Check if the current directory ends with '/tests'
if [[ "$current_dir" == */tests ]]; then
    echo "Current directory is inside 'tests'. Moving up one level..."
    cd ..

    export LD_LIBRARY_PATH=./lib:$LD_LIBRARY_PATH
    export RUST_LOG=trace
    export RUST_BACKTRACE=full

    mkdir -p logs
    cargo build
fi

kill_all() {
    SERVICE='asimplevectors'
    if [ "$(uname)" = "Darwin" ]; then
        if pgrep -xq -- "${SERVICE}"; then
            pkill -f "${SERVICE}"
        fi
        rm -r 127.0.0.1:*.db || echo "no db to clean"
    else
        set +e # killall will error if finds no process to kill
        killall "${SERVICE}"
        set -e
    fi
}

rpc() {
    local uri=$1
    local body="$2"

    echo '---'" rpc(:$uri, $body)"
    echo "call with token $TOKEN"

    # Check if token is set
    if [ -n "$TOKEN" ]; then
        # Token is set, include Authorization header
        {
            if [ ".$body" = "." ]; then
                time curl --silent "127.0.0.1:$uri" -H "Authorization: Bearer $TOKEN"
            else
                time curl --silent "127.0.0.1:$uri" -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d "$body"
            fi
        } | {
            if type jq > /dev/null 2>&1; then
                jq
            else
                cat
            fi
        }
    else
        # Token is not set, call without Authorization header
        {
            if [ ".$body" = "." ]; then
                time curl --silent "127.0.0.1:$uri"
            else
                time curl --silent "127.0.0.1:$uri" -H "Content-Type: application/json" -d "$body"
            fi
        } | {
            if type jq > /dev/null 2>&1; then
                jq
            else
                cat
            fi
        }
    fi

    echo
    echo
}

bin=./target/$BUILD_MODE/asimplevectors

echo "Killing all running asimplevectors servers and cleaning up old data"

kill_all
sleep 1

if [ "$SINGLE_MODE" = true ]; then
    echo "Start a single-node asimplevectors server..."

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 --data_path=data/data1 --log_file=logs/atinyvectors1.log --log_level $LOGLEVEL 2>&1 > logs/n1.log &
    PID1=$!
    sleep 1
    echo "Server 1 started"

    echo "Initialize server 1 as a single-node cluster"
    sleep 2
    echo
    rpc 21001/cluster/init '{}'

    echo "Server 1 is a leader now"

    sleep 2

    echo "Get metrics from the leader"
    sleep 2
    echo
    rpc 21001/cluster/metrics
    sleep 1

else
    echo "Start 3 uninitialized asimplevectors servers..."

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 --data_path=data/data1 --log_file=logs/atinyvectors1.log --log_level $LOGLEVEL 2>&1 > logs/n1.log &
    PID1=$!
    sleep 1
    echo "Server 1 started"

    nohup ${bin} --id 2 --http-addr 127.0.0.1:21002 --rpc-addr 127.0.0.1:22002 --data_path=data/data2 --log_file=logs/atinyvectors2.log --log_level $LOGLEVEL 2>&1 >> logs/n2.log &
    sleep 1
    echo "Server 2 started"

    nohup ${bin} --id 3 --http-addr 127.0.0.1:21003 --rpc-addr 127.0.0.1:22003 --data_path=data/data3 --log_file=logs/atinyvectors3.log --log_level $LOGLEVEL 2>&1 >> logs/n3.log &
    sleep 1
    echo "Server 3 started"
    sleep 1

    echo "Initialize server 1 as a single-node cluster"
    sleep 2
    echo
    rpc 21001/cluster/init '{}'

    echo "Server 1 is a leader now"

    sleep 2

    echo "Get metrics from the leader"
    sleep 2
    echo
    rpc 21001/cluster/metrics
    sleep 1

    echo "Adding node 2 and node 3 as learners, to receive log from leader node 1"

    sleep 1
    echo
    rpc 21001/cluster/add-learner '[2, "127.0.0.1:21002", "127.0.0.1:22002"]'
    echo "Node 2 added as learner"
    sleep 1
    echo
    rpc 21001/cluster/add-learner '[3, "127.0.0.1:21003", "127.0.0.1:22003"]'
    echo "Node 3 added as learner"
    sleep 1

    echo "Get metrics from the leader, after adding 2 learners"
    sleep 2
    echo
    rpc 21001/cluster/metrics
    sleep 1

    echo "Changing membership from [1] to 3 nodes cluster: [1, 2, 3]"
    echo
    rpc 21001/cluster/change-membership '[1, 2, 3]'
    sleep 1
    echo "Membership changed"
    sleep 1

    echo "Get metrics from the leader again"
    sleep 1
    echo
    rpc 21001/cluster/metrics
    sleep 1
fi

if [ "$SINGLE_MODE" = true ]; then
    python ./example/test_$TARGET.py --single
else
    python ./example/test_$TARGET.py
fi

# Continue with the rest of the script...

echo "Killing all nodes in 3s..."
sleep 1
echo "Killing all nodes in 2s..."
sleep 1
echo "Killing all nodes in 1s..."
sleep 1
kill_all