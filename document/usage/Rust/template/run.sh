#!/bin/bash -v

cd sample
cargo build --release

cd ..
rlr judge ./sample/target/release/sample

