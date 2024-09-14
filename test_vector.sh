#!/bin/sh

set -o errexit

cargo build

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

    echo
    echo
}

export LD_LIBRARY_PATH=./lib:$LD_LIBRARY_PATH
export RUST_LOG=trace
export RUST_BACKTRACE=full
bin=./target/debug/asimplevectors

echo "Killing all running asimplevectors servers and cleaning up old data"

kill_all
sleep 1

if ls 127.0.0.1:*.db
then
    rm -r 127.0.0.1:*.db || echo "no db to clean"
fi

echo "Start 3 uninitialized asimplevectors servers..."

${bin} --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 2>&1 > n1.log &
PID1=$!
sleep 1
echo "Server 1 started"

nohup ${bin} --id 2 --http-addr 127.0.0.1:21002 --rpc-addr 127.0.0.1:22002 2>&1 >> n2.log &
sleep 1
echo "Server 2 started"

nohup ${bin} --id 3 --http-addr 127.0.0.1:21003 --rpc-addr 127.0.0.1:22003 2>&1 >> n3.log &
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
rpc 21001/cluster/add-learner       '[2, "127.0.0.1:21002", "127.0.0.1:22002"]'
echo "Node 2 added as learner"
sleep 1
echo
rpc 21001/cluster/add-learner       '[3, "127.0.0.1:21003", "127.0.0.1:22003"]'
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

echo "Create space 'spacename' on leader"
sleep 1
echo
rpc 21001/api/space  \
'{
    "name": "spacename",
    "dimension": 128,
    "metric": "cosine",
    "hnsw_config": {
        "ef_construct": 123
    },
    "quantization_config": {
        "scalar": {
            "type": "int8",
            "quantile": 0.99,
            "always_ram": true
        }
    },
    "dense": {
        "dimension": 1536,
        "metric": "Cosine",
        "hnsw_config": {
            "m": 32,
            "ef_construct": 123
        },
        "quantization_config": {
            "scalar": {
                "type": "int8",
                "quantile": 0.8
            }
        }
    },
    "sparse": {
        "metric": "Cosine"
    },
    "indexes": {
        "index1": {
            "dimension": 4,
            "metric": "Cosine",
            "hnsw_config": {
                "m": 20
            },
            "quantization_config": {
                "scalar": {
                    "type": "int8",
                    "quantile": 0.6
                }
            }
        },
        "index2": {
            "dimension": 4,
            "metric": "Cosine",
            "hnsw_config": {
                "m": 20
            },
            "quantization_config": {
                "scalar": {
                    "type": "int8",
                    "quantile": 0.6
                }
            }
        }
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
        }
    ]
}'

sleep 1
echo "Vectors upserted to 'spacename'"
sleep 1

echo "Get vectors by version ID on node 1"
rpc 21001/api/space/spacename/version/default/vectors
sleep 1

echo "Upsert vectors to 'spacename' with specific version ID"
sleep 1
rpc 21001/api/space/spacename/version/1/vector \
'{
    "vectors": [
        {
            "id": 3,
            "data": [0.9, 0.8, 0.7, 0.6],
            "metadata": {"label": "third"}
        }
    ]
}'

sleep 1
echo "Vectors upserted to 'spacename' with specific version ID"
sleep 1

echo "Get vectors by version ID 1 on node 1"
rpc 21001/api/space/spacename/version/1/vectors
sleep 1

echo "Get vectors by version ID 1 on node 2"
rpc 21002/api/space/spacename/version/1/vectors
sleep 1

echo "Get vectors by version ID 1 on node 3"
rpc 21003/api/space/spacename/version/1/vectors
sleep 1

echo "Killing all nodes in 3s..."
sleep 1
echo "Killing all nodes in 2s..."
sleep 1
echo "Killing all nodes in 1s..."
sleep 1
kill_all

rm -r 127.0.0.1:*.db
