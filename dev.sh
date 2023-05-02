#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd admin_web; trunk serve --address 0.0.0.0' & \
 bash -c 'cd server; cargo watch -- cargo shuttle run --external')
