#!/bin/bash
sudo apt-get install python3-distutils libhdf5-dev jq libssl-dev

mkdir lib
mkdir temp
cd temp
git clone https://github.com/billionvectors/atinyvectors.git
cd atinyvectors
git pull
./dockerbuild.sh --no-cache=true
cp output/* ../../lib/