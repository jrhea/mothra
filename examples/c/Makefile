SHELL := /bin/sh

include ../../config.mk

CC:=gcc
INCLUDES:=-I$(ROOT_DIR)/bindings/c/

.PHONY : examples clean

examples:
	$(shell mkdir -p $(ROOT_DIR)/bin)
	$(CC) mothra.c $(INCLUDES) -L$(ROOT_DIR)/bin/ -lmothra -o $(ROOT_DIR)/bin/mothra

clean:
	rm -rf $(ROOT_DIR)/bin/mothra*