SHELL := /bin/sh

include ../../config.mk

CFLAGS:=-O2 -fPIC
LFLAGS = -shared
INCLUDES = -I$(CBIND_DIR)/
TARGET:=$(OUT_DIR)/libmothra-ingress.$(EXT)

.PHONY: c-bindings clean	

c-bindings: 
	mkdir -p $(OUT_DIR)
	rm -f $(TARGET)
	$(CC) $(INCLUDES) $(CBIND_DIR)/mothra.c $(CFLAGS) $(LFLAGS) -o $(TARGET)

clean:
	rm -rf $(TARGET)
