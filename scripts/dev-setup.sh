#!/usr/bin/env bash

set -euxo pipefail

apt update

apt install -y cargo

# needed for cargo-tarpaulin
apt install -y pkg-config libssl-dev


