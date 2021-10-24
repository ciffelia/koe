FROM rust:1.56.0-bullseye as builder

# Switch to non-root user
RUN useradd --create-home --user-group koe
USER koe

WORKDIR /home/koe/koe
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

COPY --from=builder --chown=koe:koe /home/koe/koe/target/release/koe ./

ENTRYPOINT ["./koe"]
