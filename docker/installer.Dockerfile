# Packs the Windows installer (setup.exe) with NSIS from Linux.
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends nsis \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace/installer
