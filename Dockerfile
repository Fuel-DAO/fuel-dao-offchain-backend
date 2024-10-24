FROM debian:bookworm-20240211

WORKDIR /app

COPY ./target/x86_64-unknown-linux-musl/release/offchain_server.

RUN apt-get update \
    && apt-get install -y ca-certificates \
    && apt-get -y install curl

EXPOSE 50051
