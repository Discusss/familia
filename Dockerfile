FROM rust:1.83.0 AS builder

WORKDIR /img_build
USER root
ADD . .

RUN apt-get update && apt-get install libssl-dev pkg-config
RUN rustup override set nightly
RUN cargo build --release

# ===========================
FROM debian:12.8-slim

ARG APP=/img

# Set the timezone to europe/madrid
ENV TZ=Etc/UTC+1 \
    APP_USER=pworker

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /img_build/target/release/familia_lacabra ${APP}/familia_lacabra
COPY Rocket.toml ${APP}/Rocket.toml
COPY assets ${APP}/assets

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN ldd --version

RUN apt-get update && apt-get install -y openssl ca-certificates
RUN update-ca-certificates

USER $APP_USER
WORKDIR ${APP}

ENV RUST_LOG=info
LABEL org.opencontainers.image.source=https://github.com/Discusss/familia

ENTRYPOINT ["./familia_lacabra"]