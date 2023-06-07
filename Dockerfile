FROM rust:1.68.2-buster as builder

WORKDIR /build

COPY lib/ /build

RUN cargo build -r


FROM python:3.10-slim-buster

WORKDIR /sandbox

ENV DEPS \
        dnsutils \
        iproute2 \
        curl

RUN apt update -y; \
    apt install -y --no-install-recommends $DEPS;

COPY --from=builder /build/target/release/ethernet-echo-client /usr/local/bin
COPY --from=builder /build/target/release/ethernet-echo-server /usr/local/bin

COPY scripts/ ./scripts

RUN chmod +x -R ./scripts

CMD ["bash"]