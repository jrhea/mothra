SHELL := /bin/sh

include ../config.mk

rust: 
	cargo build --release
	cp $(CORE_DIR)/target/release/*.$(EXT) $(OUT_DIR)/

clean:
	cargo clean