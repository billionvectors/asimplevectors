#!/bin/bash
BUILD_TYPE="Release"

# Parse arguments for build type and no-cache option
for arg in "$@"
do
    case $arg in
        debug)
        BUILD_TYPE="Debug"
        shift
        ;;
        *)
        # unknown option
        ;;
    esac
done

docker build --build-arg BUILD_TYPE=$BUILD_TYPE -t asimplevectors .