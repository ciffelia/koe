FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN export DEBIAN_FRONTEND=noninteractive && \
  apt-get update && \
  apt-get install -y mold libopus-dev

# Set up cargo to use mold
RUN mkdir -p /.cargo && \
  echo '[target.x86_64-unknown-linux-gnu]' >> /.cargo/config.toml && \
  echo 'rustflags = ["-C", "link-arg=-fuse-ld=mold"]' >> /.cargo/config.toml

USER vscode

# Setup mise
RUN curl -fsSL https://mise.run | sh && \
  echo 'eval "$(mise activate bash --shims)"' >> ~/.bash_profile && \
  echo 'eval "$(mise activate bash)"' >> ~/.bashrc

# Install nightly rustfmt
RUN rustup toolchain install nightly --profile minimal --component rustfmt

# Install Claude Code
RUN curl -fsSL https://claude.ai/install.sh | bash
