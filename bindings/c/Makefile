SHELL := /bin/sh

include ../../config.mk

CC = gcc
CFLAGS = -O2 -fPIC
LFLAGS = -shared

.PHONY : bindings clean

c-bindings:
	$(shell mkdir -p $(ROOT_DIR)/bin)
	$(CC) mothra.c -L$(ROOT_DIR)/$(CORE_DIR)/target/release -lmothra $(CFLAGS) $(LFLAGS) -o $(ROOT_DIR)/bin/libmothra.$(EXT)

clean:
	rm -rf $(ROOT_DIR)/bin/mothra*