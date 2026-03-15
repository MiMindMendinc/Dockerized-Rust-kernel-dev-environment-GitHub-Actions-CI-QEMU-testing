# syntax=docker/dockerfile:1
FROM rust:nightly-slim-bookworm

# Install system dependencies:
#   qemu-system-x86  – headless kernel testing
#   llvm / lld       – linker toolchain
#   binutils         – objcopy, nm, etc.
RUN apt-get update && apt-get install -y --no-install-recommends \
    qemu-system-x86 \
    llvm \
    lld \
    binutils \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Add Rust nightly components required to compile core from source and to
# produce raw binary output (needed by cargo-bootimage).
RUN rustup component add rust-src llvm-tools-preview

# Install cargo-bootimage – creates a bootable disk image from the kernel ELF.
RUN cargo install cargo-bootimage --locked

# Create workspace directory first so we can chown it to the non-root user.
WORKDIR /workspace
RUN useradd -m -u 1000 kernel && chown -R kernel:kernel /workspace

USER kernel

ENV RUST_BACKTRACE=1
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

CMD ["/bin/bash"]
