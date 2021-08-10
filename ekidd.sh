#!/usr/bin/env bash

set -euo pipefail

df='
FROM docker.io/library/ubuntu:21.04
RUN apt-get update && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get install -yq \
        build-essential \
        clang \
        cmake \
        curl \
        file \
        git \
		musl-dev \
		musl-tools \
        sudo \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly --profile default --no-modify-path && \
    rustup target add x86_64-unknown-linux-musl && \
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
WORKDIR /root/src
'

# Good part about podman? no need to worry about PIDs and file owners.
# Bad part? Build cache is dead slow...
dfh=$(sha256sum <<<"$df" - | cut -d\  -f1)
tag=rust-wasi-builder-$dfh
if ! podman image list --format "{{.Repository}}" | grep -q $tag; then
	podman build -t $tag - <<<"$df"
fi

root="$(realpath "$(dirname "$0")")"

mkdir -p "$root/emk-target" "$root/emk-cache"

#set -x
podman run --rm -ti \
	-w "/root/src/pee" \
	-v "$root:/root/src/pee:ro" \
	-v "$root/emk-target:/root/src/pee/target" \
	-v "$root/emk-cache:/root/.cargo/registry" \
	$tag cargo +nightly build -Z build-std --release --locked --target=x86_64-unknown-linux-musl

