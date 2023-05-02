#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd admin_web
CARGO_TARGET_DIR=../static trunk build --release --public-url /
popd


cargo shuttle deploy --allow-dirty
