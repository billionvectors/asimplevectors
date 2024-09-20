#!/bin/sh

set -o errexit

# Check for the '--single' parameter
SINGLE_NODE=false
if [ "$2" = "--single" ] || [ "$3" = "--single" ]; then
    SINGLE_NODE=true
    echo "Running in single node mode"
fi

BUILD_MODE="debug"
# Check if '--release' parameter is provided
if [ "$2" = "--release" ] || [ "$3" = "--release" ]; then
  BUILD_MODE="release"
fi

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

if ls 127.0.0.1:*.db
then
    rm -r 127.0.0.1:*.db || echo "no db to clean"
fi

echo "Start a single-node asimplevectors server..."

${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 2>&1 > logs/n1.log &
PID1=$!
sleep 1
echo "Server 1 started"

echo "Initialize server 1 as a single-node cluster"
sleep 2
echo
rpc 21001/cluster/init '{}'

echo "Server 1 is a leader now"
sleep 2

echo "Create space 'spacename' on leader with error json1"
sleep 1
echo
rpc 21001/api/space \
'{
    "name": "spacename",
    "dimension": 4,
    "metric": "L2",
    "hnsw_config": {
        "M": 16,
        "ef_construct": 100
    },
}'

echo "Create space 'spacename' on leader with error json2"
sleep 1
echo
rpc 21001/api/space  \
'{
    "name": "spacename",
    "dimension": 4,
    "metric": "L2",
    "hnsw_config": {
        "M": 16,
        "ef_construct": 100'

sleep 1
echo "Space 'spacename' not created"
sleep 1

echo "Upsert vectors to wrong 'spacename'"
sleep 1
rpc 21001/api/space/dasdaad/vector \
'{
    "vectors": [
        {
            "id": 1,
            "data": [0.1, 0.2, 0.3, 0.4],
            "metadata": {"label": "first"}
        },
        {
            "id": 2,
            "data": [0.5, 0.6, 0.7, 0.8],
            "metadata": {"label": "second"}
        },
        {
            "id": 3,
            "data": [0.9, 0.8, 0.7, 0.6],
            "metadata": {"label": "third"}
        },
        {
            "id": 4,
            "data": [1.0, 0.1, 0.2, 0.3],
            "metadata": {"label": "forth"}
        },
        {
            "id": 5,
            "data": [0.2, 0.3, 0.4, 0.3],
            "metadata": {"label": "fivth"}
        }
    ]
}'

echo "Search vectors with specific version id on node 1"
rpc 21001/api/space/spacename/version/1/search \
'{
    "vector": [0.2, 0.3, 0.4, 0.3]
}'
sleep 1

echo "Killing all nodes in 3s..."
sleep 1
echo "Killing all nodes in 2s..."
sleep 1
echo "Killing all nodes in 1s..."
sleep 1
kill_all

rm -r 127.0.0.1:*.db
