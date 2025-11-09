FROM ghcr.io/ciffelia/rust-devcontainer

RUN export DEBIAN_FRONTEND=noninteractive && \
  apt-get update && \
  apt-get install -y libopus-dev

USER vscode

# Setup mise
RUN curl -fsSL https://mise.run | sh && \
  echo 'eval "$(mise activate bash --shims)"' >> ~/.bash_profile && \
  echo 'eval "$(mise activate bash)"' >> ~/.bashrc
