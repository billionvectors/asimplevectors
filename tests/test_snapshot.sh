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

if [ "$SINGLE_NODE" = true ]; then
    echo "Start a single-node asimplevectors server..."

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 --data_path=data/data1 --log_file=logs/atinyvectors1_1.log 2>&1 > logs/n1.log &
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

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 --data_path=data/data1 --log_file=logs/atinyvectors1_1.log 2>&1 > logs/n1.log &
    PID1=$!
    sleep 1
    echo "Server 1 started"

    nohup ${bin} --id 2 --http-addr 127.0.0.1:21002 --rpc-addr 127.0.0.1:22002 --data_path=data/data2 --log_file=logs/atinyvectors2_1.log 2>&1 >> logs/n2.log &
    sleep 1
    echo "Server 2 started"

    nohup ${bin} --id 3 --http-addr 127.0.0.1:21003 --rpc-addr 127.0.0.1:22003 --data_path=data/data3 --log_file=logs/atinyvectors3_1.log 2>&1 >> logs/n3.log &
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

echo "Create space 'spacename' on leader"
sleep 1
echo
rpc 21001/api/space  \
'{
    "name": "spacename",
    "dimension": 4,
    "metric": "L2",
    "hnsw_config": {
        "M": 16,
        "ef_construct": 100
    }
}'

sleep 1
echo "Space 'spacename' created"
sleep 1

echo "Upsert vectors to 'spacename' without specifying version"
sleep 1
rpc 21001/api/space/spacename/vector \
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

echo "Create snapshot version id on node 1"
rpc 21001/api/snapshot \
'{
    "spacename": 1
}'
sleep 1

echo "List snapshot version id on node 1"
rpc 21001/api/snapshots
sleep 1

response=$(curl -s http://127.0.0.1:21001/api/snapshots)
FILENAME=$(echo "$response" | jq -r 'fromjson | .snapshots[0].file_name')
DATETIME=$(echo "$FILENAME" | sed -n 's/^snapshot-\([0-9]*\)\.zip$/\1/p')
echo "Extracted FILENAME: $FILENAME DATETIME: $DATETIME"
sleep 1

echo "Download snapshot version id on node 1"
echo "URL: http://127.0.0.1:21001/api/snapshot/$DATETIME/download"
curl -OJ http://127.0.0.1:21001/api/snapshot/$DATETIME/download
mkdir -p temp
mv *.zip temp/
sleep 1

echo "Upsert vectors to 'spacename' without specifying version"
rpc 21001/api/space/spacename/vector \
'{
    "vectors": [
        {
            "id": 1,
            "data": [1.0, 1.0, 1.0, 1.0],
            "metadata": {"label": "modified"}
        }
    ]
}'
sleep 1

echo "Search vectors with specific version id on node 1"
rpc 21001/api/space/spacename/version/1/search \
'{
    "vector": [0.1, 0.2, 0.3, 0.4]
}'
sleep 1

echo "Restore version id on node 1"
rpc 21001/api/snapshot/$DATETIME/restore \
'{
}'
sleep 1

echo "Search vectors with specific version id on node 1"
rpc 21001/api/space/spacename/version/1/search \
'{
    "vector": [0.1, 0.2, 0.3, 0.4]
}'
sleep 1

if [ "$SINGLE_NODE" = false ]; then
    echo "Search vectors with specific version id on node 2"
    rpc 21002/api/space/spacename/version/1/search \
    '{
        "vector": [0.1, 0.2, 0.3, 0.4]
    }'
    sleep 1

    echo "Search vectors with specific version id on node 3"
    rpc 21003/api/space/spacename/version/1/search \
    '{
        "vector": [0.1, 0.2, 0.3, 0.4]
    }'
    sleep 1
fi

echo "Killing all nodes in 3s..."
sleep 1
echo "Killing all nodes in 2s..."
sleep 1
echo "Killing all nodes in 1s..."
sleep 1
kill_all

echo "Restart Nodes..."
sleep 1

if ls 127.0.0.1:*.db
then
    rm -r 127.0.0.1:*.db || echo "no db to clean"
fi

if [ "$SINGLE_NODE" = true ]; then
    echo "Start a single-node asimplevectors server..."

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 --data_path=data/data1 --log_file=logs/atinyvectors1_2.log 2>&1 > logs/n1.log &
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

    ${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001  --data_path=data/data1 --log_file=logs/atinyvectors1_2.log 2>&1 > logs/n1.log &
    PID1=$!
    sleep 1
    echo "Server 1 started"

    nohup ${bin} --id 2 --http-addr 127.0.0.1:21002 --rpc-addr 127.0.0.1:22002 --data_path=data/data2 --log_file=logs/atinyvectors2_2.log  2>&1 >> logs/n2.log &
    sleep 1
    echo "Server 2 started"

    nohup ${bin} --id 3 --http-addr 127.0.0.1:21003 --rpc-addr 127.0.0.1:22003 --data_path=data/data3 --log_file=logs/atinyvectors3_2.log  2>&1 >> logs/n3.log &
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

echo "Restore version id on node 1"
rpc 21001/api/snapshot/$DATETIME/restore \
'{
}'
sleep 1

echo "Search vectors with specific version id on node 1"
rpc 21001/api/space/spacename/version/1/search \
'{
    "vector": [0.1, 0.2, 0.3, 0.4]
}'
sleep 1

if [ "$SINGLE_NODE" = false ]; then
    echo "Search vectors with specific version id on node 2"
    rpc 21002/api/space/spacename/version/1/search \
    '{
        "vector": [0.1, 0.2, 0.3, 0.4]
    }'
    sleep 1

    echo "Search vectors with specific version id on node 3"
    rpc 21003/api/space/spacename/version/1/search \
    '{
        "vector": [0.1, 0.2, 0.3, 0.4]
    }'
    sleep 1
fi

echo "Killing all nodes in 3s..."
sleep 1
echo "Killing all nodes in 2s..."
sleep 1
echo "Killing all nodes in 1s..."
sleep 1
kill_all


rm -r 127.0.0.1:*.db
