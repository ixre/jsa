#!/usr/bin/env bash

# cargo build --release --target x86_64-unknown-linux-gnu
# docker build -t jarry6/jsa . && docker push jarry6/jsa:latest

docker run --rm -it -v $(pwd):/home/rust/src ekidd/rust-musl-builder \
cargo build --release --target x86_64-unknown-linux-musl

