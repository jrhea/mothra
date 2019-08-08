SHELL := /bin/sh

include config.mk

.PHONY : all c-bindings rust
.DEFAULT_GOAL : all

mash: clean all

all: c-bindings examples

examples: rust
	cd $(EXAMPLES_DIR) && make $@

c-bindings: rust
	cd $(BIND_DIR) && make $@

rust: 
	cd $(CORE_DIR) && make $@

clean:
	cd $(CORE_DIR) && make $@
	cd $(BIND_DIR) && make $@