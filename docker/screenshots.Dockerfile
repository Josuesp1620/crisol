# Renders the designer without a screen, with the Mesa software Vulkan driver (lavapipe),
# for visual checks: make screenshots writes target/screenshots/*.png.
FROM crisol-core:1.98.1

RUN apt-get update \
    && apt-get install -y --no-install-recommends mesa-vulkan-drivers libvulkan1 libegl1 \
    && rm -rf /var/lib/apt/lists/*
