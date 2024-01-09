#!/bin/bash
mkdir -p tmp

cargo clippy --all-targets --all-features -- -D warnings

cargo test

rm -rf tmp
