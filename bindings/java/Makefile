SHELL := /bin/sh

include ../../config.mk

JAVA_INCLUDES = -I$(JAVA_HOME)/include/$(OS) -I$(JAVA_HOME)/include
CFLAGS = -O2 -fPIC
LFLAGS = -shared
TARGET:=$(OUT_DIR)/libmothra-java.$(EXT)

.PHONY : bindings clean

java-bindings:
	$(shell mkdir -p $(OUT_DIR))
	$(CC) *.c $(JAVA_INCLUDES) -L$(CORE_DIR)/target/release -lmothra $(CFLAGS) $(LFLAGS) -o $(TARGET)

clean:
	rm -rf $(OUT_DIR)/mothra*