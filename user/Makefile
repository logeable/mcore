RELEASE_TARGET=target/riscv64gc-unknown-none-elf/release

SRC=$(notdir $(wildcard src/bin/*.rs))
BINS=$(patsubst %.rs, %.bin, $(SRC))

bin/%:
	cargo build --release --bin $*

%.bin: bin/%
	rust-objcopy --strip-all ${RELEASE_TARGET}/$* -O binary $(RELEASE_TARGET)/$*.bin

bins: $(BINS)

clean:
	cargo clean
