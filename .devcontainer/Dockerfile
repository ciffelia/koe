# syntax=docker/dockerfile:1
FROM mcr.microsoft.com/devcontainers/rust:2-bookworm

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y mold libopus-dev

# Set up cargo to use mold
COPY <<EOF /.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
EOF

USER vscode

# Setup mise
RUN curl -fsSL https://mise.run | sh && \
    echo 'eval "$(mise activate bash --shims)"' >> ~/.bash_profile && \
    echo 'eval "$(mise activate bash)"' >> ~/.bashrc

# Install nightly rustfmt
RUN rustup toolchain install nightly --profile minimal --component rustfmt
