FROM rust:nightly

RUN apt-get update && apt-get install -y qemu-system-x86
RUN rustup target add x86_64-unknown-none
RUN cargo install bootimage

WORKDIR /kernel
COPY . .

CMD ["sh", "-c", "cargo bootimage && qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-mimindmend-rust-kernel.bin -display none -serial stdio"]