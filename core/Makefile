SHELL := /bin/sh

include ../config.mk

JAVA_INCLUDES = -I$(JAVA_HOME)/include/$(OS) -I$(JAVA_HOME)/include
JAVA_LIBS = -L$(JAVA_HOME)/lib/server/ -ljvm

#c-bindings:
#	rm -f $(CORE_DIR)/target/release/libmothra.$(EXT)
#	cargo build --release
#	$(shell mkdir -p $(OUT_DIR))
#	mv $(CORE_DIR)/target/release/libmothra.$(EXT) $(OUT_DIR)/libmothra-c.$(EXT)
#	rm $(CORE_DIR)/target/release/libmothra*

java-bindings-ingress:
	#RUSTFLAGS="-C link-args=-Wl,-rpath,/Library/Java/JavaVirtualMachines/openjdk-11.0.1.jdk/Contents/Home/lib/server" cargo build --release
	cargo build --release
	$(shell mkdir -p $(OUT_DIR))
	mv $(CORE_DIR)/target/release/libmothra.$(EXT) $(CORE_DIR)/target/release/libmothra-java.$(EXT)
	cp $(CORE_DIR)/target/release/*.* $(OUT_DIR)/
clean:
	cargo clean