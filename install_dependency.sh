#!/bin/bash
sudo apt-get install -y python3-distutils libhdf5-dev jq libssl-dev libomp-dev

mkdir lib
mkdir temp
cd temp
git clone https://github.com/billionvectors/atinyvectors.git
cd atinyvectors
git pull
./dockerbuild.sh --opt-level=avx2 --no-cache=true
cp output/* ../../lib/
cp -rf db ../../