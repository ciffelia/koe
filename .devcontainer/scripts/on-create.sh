#!/usr/bin/env bash
set -euxo pipefail

rustup toolchain install nightly --component rustfmt

curl -fsSL https://claude.ai/install.sh | bash

# Workaround for Claude Code IDE integration
ln -s "$CONTAINER_WORKSPACE_FOLDER/.devcontainer/persist/.claude" "$HOME/.claude"
