SHELL := /bin/sh

include ../../config.mk

JAVA_HOME = $(shell java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | sed 's/\s*java.home = //' | sed 's/\/jre//')
JAVA_INCLUDES = -I$(JAVA_HOME)/include/$(OS) -I$(JAVA_HOME)/include
CC = gcc
CFLAGS = -O2 -fPIC
LFLAGS = -shared

.PHONY : bindings clean

java-bindings:
	$(shell mkdir -p $(ROOT_DIR)/bin)
	$(CC) *.c $(JAVA_INCLUDES) -L$(ROOT_DIR)/$(CORE_DIR)/target/release -lmothra $(CFLAGS) $(LFLAGS) -o $(ROOT_DIR)/bin/libmothrajni.$(EXT)

clean:
	rm -rf $(ROOT_DIR)/bin/mothra*