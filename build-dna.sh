#!/bin/bash

set -xe

cd dna/zomes/integrity
cargo build --release --target wasm32-unknown-unknown
cd ../coordinator
cargo build --release --target wasm32-unknown-unknown
cd ../../workdir/dna
hc dna pack . -o dna.dna
# cd ../happ
# hc app pack . -o happ.happ
cd ../..
