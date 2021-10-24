FROM lukemathwalker/cargo-chef:0.1.31-rust-1.56.0-bullseye AS chef

# Switch to non-root user
RUN useradd --create-home --user-group koe
USER koe

WORKDIR /home/koe/app

###

FROM chef AS planner

COPY --chown=koe:koe . .

RUN cargo chef prepare --recipe-path recipe.json

###

FROM chef as builder

COPY --from=planner /home/koe/app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY --chown=koe:koe . .

RUN cargo build --release --bin koe

###

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*

# Switch to non-root user
RUN useradd --create-home --user-group koe
USER koe
WORKDIR /home/koe

COPY --from=builder --chown=koe:koe /home/koe/app/target/release/koe ./

ENTRYPOINT ["./koe"]
