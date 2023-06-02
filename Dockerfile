FROM rust:1.68.2-buster as builder

WORKDIR /build

COPY lib/ /build

RUN cargo build -r

#FROM rust:1.68.2-buster
#
#RUN apt update -y && apt upgrade -y && \
#    apt install -y dnsutils iproute2