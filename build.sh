#!/bin/bash

usage() {
  echo "Usage: $0 [--release]"
  echo "Please provide a test name as an argument."
  exit 1
}

# Check if a parameter was provided
if [ -z "$1" ]; then
  echo "Error: No parameter provided."
  usage
fi

BUILD_MODE=""

# Check if '--release' parameter is provided
if [ "$1" = "--release" ] ; then
  BUILD_MODE="--release"
fi

cargo build $BUILD_MODE
cp -rf ./target/$BUILD_MODE/asimplevectors ./
