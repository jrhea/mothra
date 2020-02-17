SHELL := /bin/sh

include ../../config.mk

CFLAGS:=-O2 -fPIC
LFLAGS =-v
INCLUDES = -I$(CBIND_DIR)/
OBJ:=$(OUT_DIR)/libmothra-ingress.obj
TARGET:=$(OUT_DIR)/libmothra-ingress.a

.PHONY: c-bindings clean	

c-bindings: 
	mkdir -p $(OUT_DIR)
	rm -f $(TARGET)
	$(CC) $(INCLUDES) -c $(CBIND_DIR)/mothra-ingress.c $(CFLAGS) -o $(OBJ)
	$(AR) -r $(LFLAGS) $(TARGET) $(OBJ)

clean:
	rm -rf $(TARGET)
	rm -rf $(OBJ)
