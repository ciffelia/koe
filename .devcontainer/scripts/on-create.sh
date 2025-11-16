#!/usr/bin/env bash
set -euxo pipefail

# Workaround for Claude Code IDE integration
ln -s "$CONTAINER_WORKSPACE_FOLDER/.devcontainer/persist/.claude" "$HOME/.claude"

# Install rust toolchain
rustup toolchain install

# Setup mise
mise trust
mise install
