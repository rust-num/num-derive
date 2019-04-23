#!/bin/bash

set -ex

echo Testing num-derive on rustc ${TRAVIS_RUST_VERSION}

# num-derive should build and test everywhere.
cargo build --verbose --features="$FEATURES"

# Some cargo versions were buggy about passing dev-deps to rustdoc,
# but worked when docs were tested separately.
case "$TRAVIS_RUST_VERSION" in
    1.20.0 | 1.26.0 )
        cargo test --verbose --features="$FEATURES" --tests
        cargo test --verbose --features="$FEATURES" --doc
        ;;
    *)
        cargo test --verbose --features="$FEATURES"
        ;;
esac
