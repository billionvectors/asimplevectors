#!/bin/bash

usage() {
  echo "Usage: $0 [--debug|--release] [--loglevel=debug|info|error]"
  echo "Please provide a test name as an argument."
  exit 1
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

CARGO_BUILD_MODE="--release"
LOGLEVEL=""
SERVICELOGLEVEL=""
BUILD_MODE="release"

for arg in "$@"; do
  case $arg in
    --debug)
      CARGO_BUILD_MODE=""
      LOGLEVEL="--log_level=debug"
      SERVICELOGLEVEL="--service_log_level=debug"
      BUILD_MODE="debug"
      echo "Debug Build mode"
      ;;
    --release)
      CARGO_BUILD_MODE="--release"
      echo "Release Build mode"
      ;;
    --loglevel=*)
      LOGLEVEL="--log_level=${arg#*=}"
      SERVICELOGLEVEL="--service_log_level=${arg#*=}"
      echo "Log level set to $LOGLEVEL"
      ;;
  esac
done

# Check if the dependency installed
if [ ! -d "lib" ]; then
  echo "Dependency Library not found. install dependency..."
  bash ./install_dependency.sh
fi

export RUST_LOG=trace
export RUST_BACKTRACE=full

rm -rf logs
rm -rf data
rm -rf temp
mkdir -p logs

# Build with or without --release based on input
cargo build $CARGO_BUILD_MODE

# Check if the virtual environment folder exists
if [ ! -d "venv" ]; then
  echo "Virtual environment not found. Creating virtual environment..."
  
  # Create virtual environment
  sudo apt install python3
  sudo apt install python3-venv
  python3 -m venv venv

  if [ $? -eq 0 ]; then
    echo "Virtual environment created successfully."
  else
    echo "Error creating virtual environment."
    exit 1
  fi
fi

# Activate the virtual environment
echo "Activating virtual environment..."
source venv/bin/activate
pip install --upgrade pip setuptools wheel

# Install dependencies from requirements.txt
if [ -f "./example/requirements.txt" ]; then
  echo "Installing dependencies from requirements.txt..."
  pip install -r ./example/requirements.txt

  if [ $? -eq 0 ]; then
    echo "Dependencies installed successfully."
  else
    echo "Error installing dependencies."
    exit 1
  fi
fi

# Check if .env file exists
if [ ! -f ".env" ]; then
  echo "Error: .env file not found."
  echo "Please copy .env.local to .env using the following command:"
  echo "cp -rf .env.local .env"
  exit 1
fi

rm -r 127.0.0.1:*.db
rm -r 0.0.0.0:*.db

cargo build $CARGO_BUILD_MODE

bin=./target/$BUILD_MODE/asimplevectors

${bin} --id 1 --http-addr 0.0.0.0:21001 --rpc-addr 0.0.0.0:22001 --data_path=data/data1 --log_file=logs/atinyvectors1.log $LOGLEVEL $SERVICELOGLEVEL 2>&1 > logs/n1.log &
sleep 1
echo "Server 1 started"

nohup ${bin} --id 2 --http-addr 0.0.0.0:21002 --rpc-addr 0.0.0.0:22002 --data_path=data/data2 --log_file=logs/atinyvectors2.log $LOGLEVEL $SERVICELOGLEVEL 2>&1 >> logs/n2.log &
sleep 1
echo "Server 2 started"

nohup ${bin} --id 3 --http-addr 0.0.0.0:21003 --rpc-addr 0.0.0.0:22003 --data_path=data/data3 --log_file=logs/atinyvectors3.log $LOGLEVEL $SERVICELOGLEVEL 2>&1 >> logs/n3.log &
sleep 1
echo "Server 3 started"
sleep 1

echo "Initialize server 1 as a single-node cluster"
sleep 2
echo
rpc 21001/cluster/init '{}'

echo "Server 1 is a leader now"

sleep 2

echo "Adding node 2 and node 3 as learners, to receive log from leader node 1"
rpc 21001/cluster/add-learner '[2, "127.0.0.1:21002", "127.0.0.1:22002"]'
sleep 1

rpc 21001/cluster/add-learner '[3, "127.0.0.1:21003", "127.0.0.1:22003"]'
sleep 1