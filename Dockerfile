FROM rust:1.73.0-bullseye as builder

RUN apt-get update && \
    apt-get install -y libopus-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /root/koe
COPY . .

RUN --mount=type=cache,target=/root/.cargo/bin \
    --mount=type=cache,target=/root/.cargo/registry/index \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/koe/target \
    cargo build --release --bin koe && \
    cp target/release/koe /usr/local/bin/koe

###

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates ffmpeg && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/koe /usr/local/bin/koe

# Switch to unpriviledged user
RUN useradd --create-home --user-group koe
USER koe
WORKDIR /home/koe

ARG SENTRY_RELEASE
ENV SENTRY_RELEASE=$SENTRY_RELEASE

ENTRYPOINT ["koe"]
