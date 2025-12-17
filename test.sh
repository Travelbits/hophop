#!/bin/sh

# Kept in a shell script to be easily portable to no-GitHub CI systems.
#
# This expects the Ariel OS "getting started" setup to be present, and suitable
# caching options to be set.

set -ex

for DIR in ts-103-636-utils ts-103-636-utils
do
    cd "${DIR}"
    RUSTFLAGS="-D warnings" cargo check
    RUSTFLAGS="-D warnings" cargo check --all-features
    cargo clippy -- --deny clippy::all --deny clippy::pedantic
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features
    cargo fmt --check
    cargo test
    cargo test --all-features
    cd ..
done

# Initially those do build tests only; turning clippy and checks on is a good
# next step, but only once these stabilize a little.

cd examples
# FIXME: Going through `run` but not really -- because a plain build fails due to the multiple binaries.
for EX in rx tx rssi
do
    laze build -b nrf9151-dk -D LOG=trace -D CARGO_RUNNER=true run --bin ${EX}
done
