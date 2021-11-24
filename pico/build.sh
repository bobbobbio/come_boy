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

if [ ! -f "rom.bin" ]; then
    echo "Missing ROM for pico build. symlink rom.bin to some valid ROM"
    exit 1
fi

export RUSTC=$PROJECT_ROOT/pico/pgo/bin/rustc
LLVM_BIN="$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin"
PROFDATA="$LLVM_BIN/llvm-profdata"

TARGET_DIR="$PROJECT_ROOT/target"
PGO_DIR="$TARGET_DIR/pico-pgo-data"

rm -rf $PGO_DIR
mkdir -p $PGO_DIR

export RUSTFLAGS="-C profile-generate=$PGO_DIR"

cargo build \
    --release \
    --no-default-features \
    --package come_boy_pico

rustc pgo/benchmark.rs \
    -o $TARGET_DIR/pico_pgo_benchmark \
    -C opt-level=3 \
    -C lto=thin \
    -C profile-generate=$PGO_DIR \
    -C panic=abort \
    -C default-linker-libraries \
    -C link-arg=-L$TARGET_DIR/release \
    -C link-arg=-lm \
    -C link-arg=-zmuldefs

rm -rf $PGO_DIR
mkdir -p $PGO_DIR

$TARGET_DIR/pico_pgo_benchmark

PGO_FILE=$TARGET_DIR/pico-pgo.profdata
$PROFDATA merge -o $PGO_FILE $PGO_DIR
$PROFDATA show $PGO_FILE

export RUSTFLAGS="-C profile-use=$PGO_FILE"
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
