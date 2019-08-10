SHELL := /bin/sh

include ../config.mk

rust: c
	rm -f $(CORE_DIR)/target/release/*.$(EXT)
	cargo build --release
	cp $(CORE_DIR)/target/release/*.$(EXT) $(OUT_DIR)/

c:
	mkdir -p $(CORE_DIR)/target/release
	rm -f $(CORE_DIR)/target/release/liblibp2p_receive_gossip.a
	$(CC) -c $(CORE_DIR)/src/c/libp2p_receive_gossip.c -o $(CORE_DIR)/target/release/liblibp2p_receive_gossip.a

clean:
	cargo clean