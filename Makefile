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

c-bindings:
	# @echo ""
	# @echo Building C bindings
	# cd $(BIND_DIR) && make $@
	# @echo ""
	# @echo Building Rust bindings
	# cd $(CORE_DIR) && make $@

java-bindings: java-bindings-ingress java-bindings-egress

java-bindings-ingress:
	@echo ""
	@echo Building Java bindings
	cd $(BIND_DIR) && make $@
	@echo ""
	@echo Building Rust bindings
	cd $(CORE_DIR) && make $@

java-bindings-egress:
	@echo ""
	@echo Building Java bindings
	cd $(BIND_DIR) && make $@

clean:
	cd $(CORE_DIR) && make $@
	cd $(BIND_DIR) && make $@
	cd $(EXAMPLES_DIR) && make $@