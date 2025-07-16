#!/bin/bash

echo building for windows
cargo build --release --target "x86_64-pc-windows-gnu"

echo building for linux
cargo build --release --target "x86_64-unknown-linux-gnu"
