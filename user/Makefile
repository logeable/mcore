RELEASE_TARGET=target/riscv64gc-unknown-none-elf/release
helloworld:
	cargo build --release --bin helloworld

helloworld.bin: helloworld
	rust-objcopy --strip-all ${RELEASE_TARGET}/$< -O binary ${RELEASE_TARGET}/$@

build:
	cargo build --release

clean:
	cargo clean
