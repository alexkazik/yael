#!/bin/bash

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly` and `+stable` is used, `+stable` or `+beta` are the most common alternatives

TOOLCHAIN=${1:-+stable}
echo Using toolchain $TOOLCHAIN

# builds
cargo $TOOLCHAIN build --locked --release || exit 1

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds
cargo $TOOLCHAIN build --locked --release --tests || exit 1

# clippy
cargo $TOOLCHAIN clippy --locked --release --tests -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
cargo rdme --force || exit 1

TOOLCHAIN=${1:-+stable}
echo Using toolchain $TOOLCHAIN

# create docs
cargo $TOOLCHAIN doc || exit 1

# tests
cargo $TOOLCHAIN test --locked --release || exit 1
