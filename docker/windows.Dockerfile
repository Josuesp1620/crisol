# Cross-compiles Crisol for Windows from Linux with cargo-xwin. The first build downloads
# the Microsoft C runtime and Windows SDK headers (accepting Microsoft's license) into `.cache/`.
# Debug builds compile the HLSL shaders at run time; release builds need Windows (see CI).
FROM rust:1.98.1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends clang lld llvm cmake \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add x86_64-pc-windows-msvc \
    && rustup component add clippy \
    && cargo install --locked cargo-xwin

WORKDIR /workspace
