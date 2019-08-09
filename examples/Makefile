SHELL := /bin/sh

include ../config.mk

examples: 
	cd c && make $@
	cd java && make $@

clean:
	cd $(EXAMPLES_DIR) && make $@