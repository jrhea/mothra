SHELL := /bin/sh

include ../../config.mk

INCLUDES:=-I$(CBIND_DIR)

.PHONY : examples clean

examples:
	$(CC) example.c $(INCLUDES) -L$(OUT_DIR)/ -lmothra-c -o $(OUT_DIR)/example

clean:
	rm -rf $(OUT_DIR)/example*