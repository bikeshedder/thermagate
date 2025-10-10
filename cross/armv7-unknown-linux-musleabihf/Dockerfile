FROM ghcr.io/cross-rs/armv7-unknown-linux-musleabihf:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    clang llvm-dev libclang-dev cmake pkg-config && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
