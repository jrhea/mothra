SHELL := /bin/sh

include ../../config.mk

JAVA_INCLUDES = -I$(JAVA_HOME)/include/$(OS) -I$(JAVA_HOME)/include
JAVA_LIBS = -L$(JAVA_HOME)/lib/server -ljvm
INCLUDES = -I$(JBIND_DIR)/
CFLAGS = -O2 -fPIC
LFLAGS = -shared

.PHONY : java-bindings-ingress java-bindings-egress clean

java-bindings-ingress:
	mkdir -p $(CORE_DIR)/target/release
	rm -f $(CORE_DIR)/target/release/*.*
	$(CC) $(JBIND_DIR)/mothra-jni-ingress.c $(INCLUDES) $(JAVA_INCLUDES) $(JAVA_LIBS) $(CFLAGS) $(LFLAGS) -o $(CORE_DIR)/target/release/libmothra-ingress.$(EXT)

java-bindings-egress:
	$(shell mkdir -p $(OUT_DIR))
	$(CC) $(JBIND_DIR)/mothra-jni-egress.c $(INCLUDES) -L$(OUT_DIR) -lmothra-java $(JAVA_INCLUDES) $(JAVA_LIBS) $(CFLAGS) $(LFLAGS) -o $(OUT_DIR)/libmothra-egress.$(EXT)

clean:
	rm -rf $(OUT_DIR)/mothra*
