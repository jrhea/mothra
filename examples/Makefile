SHELL := /bin/sh

include ../config.mk

examples: 
	cd $(CBIND_DIR) && make $@
	cd $(JBIND_DIR) && make $@

clean:
	cd $(EXAMPLES_DIR) && make $@