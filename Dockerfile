FROM rust:1.68.2-buster

RUN apt update -y && apt upgrade -y && \
    apt install -y dnsutils iproute2