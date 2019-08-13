SHELL := /bin/sh

include ../../config.mk

CFLAGS:=-O2 -fPIC
INCLUDES = -I$(CBIND_DIR)/
TARGET:=$(CORE_DIR)/target/release/libmothra-bind.a

.PHONY: c-bindings clean	

c-bindings: 
	mkdir -p $(CORE_DIR)/target/release
	rm -f $(TARGET)
	$(CC) $(INCLUDES) -c $(CBIND_DIR)/mothra.c $(CFLAGS) -o $(TARGET)

clean:
	rm -rf $(OUT_DIR)/libmothra.a
