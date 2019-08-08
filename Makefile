SHELL := /bin/sh

include config.mk

all: core

core: 
	cd $(CDIR) && make $@

run: 
	cd $(CDIR) && make $@

clean:
	cd $(CDIR) && make $@