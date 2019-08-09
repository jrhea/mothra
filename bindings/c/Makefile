SHELL := /bin/sh

include ../../config.mk

CFLAGS:=-O2 -fPIC
LFLAGS:=-shared
TARGET:=$(OUT_DIR)/libmothra-c.$(EXT)

.PHONY: c-bindings clean

c-bindings:
	$(CC) mothra.c -L$(CORE_DIR)/target/release -lmothra $(CFLAGS) $(LFLAGS) -o $(TARGET)

clean:
	rm -rf $(OUT_DIR)/mothra*