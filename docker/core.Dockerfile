# Builds, lints and tests every crate on Linux, including the GPUI ones.
FROM rust:1.98.1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        clang lld cmake pkg-config \
        libfontconfig-dev libfreetype-dev libwayland-dev libxkbcommon-x11-dev \
        libx11-xcb-dev libxcb1-dev libvulkan1 libssl-dev libzstd-dev libglib2.0-dev \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add clippy rustfmt

WORKDIR /workspace
