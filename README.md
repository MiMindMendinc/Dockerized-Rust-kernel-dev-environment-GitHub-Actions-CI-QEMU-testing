# Dockerized-Rust-kernel-dev-environment-GitHub-Actions-CI-QEMU-testing
Adds Docker dev environment for Rust kernel project:  - Dockerfile: Rust nightly + QEMU + bootimage - docker-compose.yml: interactive dev + Cargo cache - .dockerignore - GitHub Actions CI: build bootimage, QEMU test, artifact upload  Reproducible builds/tests, non-root user, fast iteration.  Tested locally. Ready for merge.
