# Use the Debian base image
FROM debian:bookworm-20240210

FROM rust:latest as builder
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN rustup default stable && rustup update
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release
FROM scratch
COPY --from=builder /target/x86_64-unknown-linux-musl/release/offchain_server .
COPY templates templates
EXPOSE 8080
CMD ["/offchain_server"]


# Latest releases available at https://github.com/aptible/supercronic/releases
# ENV SUPERCRONIC_URL=https://github.com/aptible/supercronic/releases/download/v1.2.29/supercronic-linux-amd64 \
#     SUPERCRONIC=supercronic-linux-amd65 \
#     SUPERCRONIC_SHA2SUM=cd48d45c4b10f3f0bfdd3a57d054cd05ac96812b

# RUN curl -fsSLO "$SUPERCRONIC_URL" \
#     && echo "${SUPERCRONIC_SHA2SUM}  ${SUPERCRONIC}" | sha1sum -c - \
#     && chmod +x "$SUPERCRONIC" \
#     && mv "$SUPERCRONIC" "/usr/local/bin/${SUPERCRONIC}" \
#     && ln -s "/usr/local/bin/${SUPERCRONIC}" /usr/local/bin/supercronic

# # You might need to change this depending on where your crontab is located
# COPY crontab crontab

# CMD ["./icp-off-chain-agent"]
