FROM rust:1.68.2-buster as builder

WORKDIR /build

COPY lib/ /build

RUN cargo build -r

FROM python:3.10-slim-buster

COPY --from=builder /build/target/release/ethernet-echo-client /usr/local/bin
COPY --from=builder /build/target/release/ethernet-echo-server /usr/local/bin

RUN apt update -y && apt upgrade -y && \
    apt install -y dnsutils iproute2 curl