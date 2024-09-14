#!/bin/bash
mkdir lib
mkdir temp
cd temp
git clone https://github.com/billionvectors/atinyvectors_dev.git
cd atinyvectors_dev
git pull
# ./dockerbuild.sh --no-cache=true
# cp output/* ../../lib/
./build_run_test.sh
cp -rf build/lib* ../../lib/

