SHELL := /bin/sh

include config.mk

.PHONY:=all examples bindings c-bindings java-bindings rust clean

all: examples bindings

mash: clean all

examples: bindings
	@echo ""
	@echo Building examples
	cd $(EXAMPLES_DIR) && make $@

bindings: c-bindings java-bindings

c-bindings: rust
	@echo ""
	@echo Building C bindings
	cd $(BIND_DIR) && make $@

java-bindings: rust
	@echo ""
	@echo Building Java bindings
	cd $(BIND_DIR) && make $@

rust:
	@echo ""
	@echo Building Rust bindings
	cd $(CORE_DIR) && make $@

clean:
	cd $(CORE_DIR) && make $@
	cd $(BIND_DIR) && make $@
	cd $(EXAMPLES_DIR) && make $@