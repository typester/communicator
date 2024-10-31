#!/bin/sh

set -e

cargo build --lib --release --target=aarch64-linux-android
cargo build --lib --release --target=x86_64-linux-android
cp target/aarch64-linux-android/release/libcommunicator.so ../android/app/src/main/jniLibs/arm64-v8a/libuniffi_communicator.so
cp target/x86_64-linux-android/release/libcommunicator.so ../android/app/src/main/jniLibs/x86_64/libuniffi_communicator.so

cargo run --features=uniffi-cli --bin uniffi-bindgen -- generate -l kotlin --library target/aarch64-linux-android/release/libcommunicator.so --out-dir ../android/app/src/main/java
