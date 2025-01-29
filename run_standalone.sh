#!/bin/bash

usage() {
  echo "Usage: $0 [--debug|--release] [--loglevel=debug|info|error]"
  echo "Please provide a test name as an argument."
  exit 1
}

CARGO_BUILD_MODE="--release"
LOGLEVEL=""
SERVICELOGLEVEL=""

for arg in "$@"; do
  case $arg in
    --debug)
      CARGO_BUILD_MODE=""
      LOGLEVEL="--log_level=debug"
      SERVICELOGLEVEL="--service_log_level=debug"
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

cargo run $CARGO_BUILD_MODE -- --standalone $LOGLEVEL $SERVICELOGLEVEL

rm -r 127.0.0.1:*.db
rm -r 0.0.0.0:*.db