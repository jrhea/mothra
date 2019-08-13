SHELL := /bin/sh

include ../../config.mk

CFLAGS:=-O2 -fPIC
LFLAGS = -shared
INCLUDES = -I$(CBIND_DIR)/
TARGET:=$(CORE_DIR)/target/release/libmothra-ingress.$(EXT)

.PHONY: c-bindings clean	

c-bindings: 
	mkdir -p $(CORE_DIR)/target/release
	rm -f $(TARGET)
	$(CC) $(INCLUDES) $(CBIND_DIR)/mothra.c $(CFLAGS) $(LFLAGS) -o $(TARGET)

clean:
	rm -rf $(OUT_DIR)/libmothra.a
