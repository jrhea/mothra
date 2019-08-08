SHELL := /bin/sh

include ../../config.mk

CC = gcc

.PHONY : examples clean

examples:
	$(shell mkdir -p $(ROOT_DIR)/bin)
	$(CC) mothra.c -L$(ROOT_DIR)/bin/ -lmothra -o $(ROOT_DIR)/bin/mothra

clean:
	rm -rf $(ROOT_DIR)/bin/mothra*