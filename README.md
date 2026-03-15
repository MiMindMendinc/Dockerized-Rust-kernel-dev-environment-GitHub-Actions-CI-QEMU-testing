# Extreme Rust Kernel – 2026 Edition

![CI](https://github.com/MiMindMendinc/Dockerized-Rust-kernel-dev-environment-GitHub-Actions-CI-QEMU-testing/actions/workflows/build-and-test.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust: nightly](https://img.shields.io/badge/Rust-nightly-orange.svg)

High-performance, `no_std` Rust kernel research project.  
Goal: multicore preemptive scheduling, async runtime, lock-free allocators,
huge pages, fast syscalls.

## Status

- ✅ Boots in QEMU (serial output verified in CI)
- ✅ Dockerised dev environment
- ✅ GitHub Actions CI — build bootimage → QEMU headless test → artefact upload

## Project Layout

```
.
├── .cargo/config.toml          ← x86_64-unknown-none target + build-std
├── .github/workflows/
│   └── build-and-test.yml      ← CI pipeline
├── src/
│   └── main.rs                 ← kernel entry point
├── Cargo.toml
├── rust-toolchain.toml         ← nightly + rust-src + llvm-tools-preview
├── Dockerfile                  ← Rust nightly + QEMU + cargo-bootimage
└── docker-compose.yml          ← interactive dev service
```

## Quick Start (Docker)

```bash
# Build the dev image once
docker compose build

# Build the kernel bootimage
docker compose run --rm kernel-dev cargo bootimage --release

# Run in QEMU (headless, serial output to terminal)
docker compose run --rm kernel-dev \
  qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-extreme-rust-kernel.bin \
  -serial stdio \
  -display none \
  -m 512M \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -no-reboot

# Interactive shell inside the container
docker compose run --rm kernel-dev bash
```

## CI/CD

GitHub Actions automatically:

1. Builds the Docker image
2. Runs `cargo bootimage --release` inside the container
3. Boots the kernel in a headless QEMU instance and checks for serial output
4. Uploads the bootimage binary as a workflow artefact (7-day retention)

See [`.github/workflows/build-and-test.yml`](.github/workflows/build-and-test.yml).

## Roadmap

- [ ] VGA text-mode driver
- [ ] Interrupt descriptor table (IDT) + exception handlers
- [ ] Physical / virtual memory manager
- [ ] Preemptive scheduler (multicore)
- [ ] Lock-free slab allocator
- [ ] Async executor (kernel-space futures)
- [ ] Fast SYSCALL/SYSRET interface

## License

MIT – see [LICENSE](LICENSE).
