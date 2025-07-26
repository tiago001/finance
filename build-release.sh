#!/bin/bash

## configure these for your environment
# cargo package name
PKG="finance"
# remote target
TARGET="x86_64-unknown-linux-gnu"
# list of assets to bundle
ASSETS=("static" "templates")
# cargo build directory
BUILD_DIR="target/${TARGET}/release/"

## ensure target toolchain is present
rustup target add $TARGET

sleep 1
## cross-compile
# cargo zigbuild --target $TARGET --release
cargo build --target $TARGET --release

sleep 4

## bundle
tar -cvzf "${PKG}.tar.gz" "${ASSETS[@]}" -C "${BUILD_DIR}" "${PKG}"