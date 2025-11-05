#!/usr/bin/env bash
set -euxo pipefail

rustup toolchain install nightly --component rustfmt
