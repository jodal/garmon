#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly SOURCE_PATH=./target/aarch64-unknown-linux-gnu/debug/garmon
readonly TARGET_HOST=garmon.o21a.jodal.no
readonly TARGET_PATH=./garmon

cargo build --target=aarch64-unknown-linux-gnu
scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
