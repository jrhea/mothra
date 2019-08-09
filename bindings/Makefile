SHELL := /bin/sh

include ../config.mk

java-bindings: c-bindings
	cd $(JBIND_DIR) && make $@

c-bindings: 
	cd $(CBIND_DIR) && make $@

clean:
	cd $(CBIND_DIR) && make $@