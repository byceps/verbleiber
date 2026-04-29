#!/bin/bash

# Build binary

IMAGE=ghcr.io/homeworkprod/rust-build-containers:rust-1.95.0-alsa

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp $IMAGE cargo build --release
