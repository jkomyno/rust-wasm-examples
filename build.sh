#!/bin/bash
DOCKER_PLATFORM="${DOCKER_PLATFORM:-linux/arm64}"

docker buildx build \
  --platform="${DOCKER_PLATFORM}" \
  --build-context rust=./rust \
  --build-context nodejs=./nodejs \
  -f Dockerfile \
  -o type=docker \
  . -t jkomyno/rust-wasm-examples \
  --progress plain

docker run \
  --platform="${DOCKER_PLATFORM}" \
  -v "$(pwd)/nodejs/demo":/opt/app/nodejs/demo \
  jkomyno/rust-wasm-examples

docker run \
  --platform="${DOCKER_PLATFORM}" \
  jkomyno/rust-wasm-examples "test:ci"
