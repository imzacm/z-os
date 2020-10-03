#! /bin/bash

VERSION="$(cat rust-toolchain)"

rustup install "$VERSION"
rustup override set "$VERSION"
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
