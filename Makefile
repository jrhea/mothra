SHELL := /bin/sh

include config.mk

.PHONY : all examples bindings c-bindings java-bindings rust
.DEFAULT_GOAL : all

mash: clean all

all: bindings examples

examples: bindings
	cd $(EXAMPLES_DIR) && make $@

bindings: c-bindings java-bindings

c-bindings: rust
	cd $(BIND_DIR) && make $@

java-bindings: rust
	cd $(BIND_DIR) && make $@

rust: 
	cd $(CORE_DIR) && make $@

clean:
	cd $(CORE_DIR) && make $@
	cd $(BIND_DIR) && make $@