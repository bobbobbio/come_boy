#!/bin/bash

set -ex

rustup target add thumbv6m-none-eabi

# Absolute path to this script
SCRIPT=$(readlink -f $0)

# Absolute path this script is in
SCRIPT_PARENT=`dirname $SCRIPT`

# Project root
PROJECT_ROOT=`dirname $SCRIPT_PARENT`

cd "$PROJECT_ROOT"

pushd pico

git submodule update --init
cargo build \
    --target thumbv6m-none-eabi \
    --release \
    --no-default-features \
    --package come_boy_pico

popd

pushd pico/pico-sdk
git submodule update --init
popd

mkdir -p target/cmake
pushd target/cmake

cmake $PROJECT_ROOT/pico

make

popd
