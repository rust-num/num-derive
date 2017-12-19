#!/bin/bash

set -ex

echo Testing num-derive on rustc ${TRAVIS_RUST_VERSION}

# num-derive should build everywhere.
cargo build --verbose

# We have no features to test...


if [ "$TRAVIS_RUST_VERSION" != nightly ]; then exit; fi

# num-derive testing requires compiletest_rs, which requires nightly
cargo test --verbose
