#!/bin/bash

set -e

CRATE=num-derive
MSRV=1.31

get_rust_version() {
  local array=($(rustc --version));
  echo "${array[1]}";
  return 0;
}
RUST_VERSION=$(get_rust_version)

check_version() {
  IFS=. read -ra rust <<< "$RUST_VERSION"
  IFS=. read -ra want <<< "$1"
  [[ "${rust[0]}" -gt "${want[0]}" ||
   ( "${rust[0]}" -eq "${want[0]}" &&
     "${rust[1]}" -ge "${want[1]}" )
  ]]
}

echo "Testing $CRATE on rustc $RUST_VERSION"
if ! check_version $MSRV ; then
  echo "The minimum for $CRATE is rustc $MSRV"
  exit 1
fi

FEATURES=(full-syntax)
echo "Testing supported features: ${FEATURES[*]}"

set -x

# test the default
cargo build
cargo test

# test each isolated feature
for feature in ${FEATURES[*]}; do
  cargo build --no-default-features --features="$feature"
  cargo test --no-default-features --features="$feature"
done

# test all supported features
cargo build --features="${FEATURES[*]}"
cargo test --features="${FEATURES[*]}"

# these CI crates keep tighter control over dependencies
cargo check --verbose --manifest-path ci/check/Cargo.toml
cargo check --verbose --manifest-path ci/import/Cargo.toml
