#! /usr/bin/env bash

set -eux

HOST_1="host_1"
HOST_2="host_2"

function create_hosts() {
    ip netns add "$1"
    ip netns add "$2"
}

function create_peer() {
    ip link add "$1_link" type veth peer name "$2_link"
    ip link set "$1_link" netns "$1"
    ip link set "$2_link" netns "$2"

    ip netns exec "$1" ip link set "$1_link" up
    ip netns exec "$2" ip link set "$2_link" up
}

create_hosts $HOST_1 $HOST_2
create_peer $HOST_1 $HOST_2