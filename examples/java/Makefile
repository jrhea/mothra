SHELL := /bin/sh

include ../../config.mk

JAVA_HOME = $(shell java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | sed 's/\s*java.home = //' | sed 's/\/jre//')
CLASS_PATH = $(ROOT_DIR)/bin/
vpath %.class $(CLASS_PATH)

NAME = mothra

examples: app.class

%.class:
	javac $*.java -d $(ROOT_DIR)/bin/

clean:
	rm -rf $(ROOT_DIR)/bin/*.class
