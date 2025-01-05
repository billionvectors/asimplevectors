#!/bin/bash
BUILD_TYPE="Release"
OPT_LEVEL="avx2"

# Parse arguments for build type and no-cache option
for arg in "$@"
do
    case $arg in
        debug)
        BUILD_TYPE="Debug"
        shift
        ;;
        --opt-level=*)
        OPT_LEVEL="${arg#*=}"
        shift
        ;;
        *)
        # unknown option
        ;;
    esac
done

echo "OPT_LEVEL is set to ${OPT_LEVEL}. Possible options are [generic, avx2, avx512]."

docker build --build-arg BUILD_TYPE=$BUILD_TYPE --build-arg OPT_LEVEL=$OPT_LEVEL -t asimplevectors .