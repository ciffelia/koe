FROM rust:1.62.1-bullseye as builder

RUN apt-get update && \
    apt-get install -y libopus-dev && \
    rm -rf /var/lib/apt/lists/*

# Switch to non-root user
RUN useradd --create-home --user-group koe
USER koe

WORKDIR /home/koe/app
COPY --chown=koe:koe . .

RUN cargo build --release --bin koe

###

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates ffmpeg && \
    rm -rf /var/lib/apt/lists/*

# Switch to non-root user
RUN useradd --create-home --user-group koe
USER koe
WORKDIR /home/koe

COPY --from=builder --chown=koe:koe /home/koe/app/target/release/koe ./

ARG SENTRY_RELEASE
ENV SENTRY_RELEASE=$SENTRY_RELEASE

ENTRYPOINT ["./koe"]
