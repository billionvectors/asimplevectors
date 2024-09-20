#!/bin/bash

usage() {
  echo "Usage: $0 [testname] [--single] [--release]"
  echo "Please provide a test name as an argument."
  exit 1
}

# Check if a parameter was provided
if [ -z "$1" ]; then
  echo "Error: No parameter provided."
  usage
fi

TEST_NAME="$1"
SINGLE_MODE=""
BUILD_MODE=""

# Check if '--single' parameter is provided
if [ "$2" = "--single" ] || [ "$3" = "--single" ]; then
  SINGLE_MODE="--single"
fi

# Check if '--release' parameter is provided
if [ "$2" = "--release" ] || [ "$3" = "--release" ]; then
  BUILD_MODE="--release"
fi

export ATV_LOG_LEVEL=info
export ATV_LOG_FILE=logs/atinyvectors.log
export LD_LIBRARY_PATH=./lib:$LD_LIBRARY_PATH
export RUST_LOG=trace
export RUST_BACKTRACE=full

rm -rf logs
rm -rf data
rm -rf temp
mkdir -p logs

# Build with or without --release based on input
cargo build $BUILD_MODE

# Check if the virtual environment folder exists
if [ ! -d "venv" ]; then
  echo "Virtual environment not found. Creating virtual environment..."
  
  # Create virtual environment
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

# Install dependencies from requirements.txt
if [ -f "./tests/requirements.txt" ]; then
  echo "Installing dependencies from requirements.txt..."
  pip install -r ./tests/requirements.txt

  if [ $? -eq 0 ]; then
    echo "Dependencies installed successfully."
  else
    echo "Error installing dependencies."
    exit 1
  fi
fi

TEST_FILE="./tests/test_$TEST_NAME.sh"
# Check if the test file exists
if [ -f "$TEST_FILE" ]; then
  echo "Running test: $TEST_FILE with option: $SINGLE_MODE"
  bash "$TEST_FILE" $SINGLE_MODE
else
  echo "Running test: ./tests/test_base.sh $TEST_NAME with option: $SINGLE_MODE $BUILD_MODE"
  bash "./tests/test_base.sh" $TEST_NAME $SINGLE_MODE $BUILD_MODE
fi
