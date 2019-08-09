OS:=$(shell uname -s | tr '[:upper:]' '[:lower:]')

ifeq ($(OS), linux)
	EXT:=so
	OS_LFLAGS:=
	JAVA_HOME:=
else ifeq ($(OS), darwin)
	EXT:=dylib
	OS_LFLAGS:=-mmacosx-version-min=$(shell defaults read loginwindow SystemVersionStampAsString) -framework CoreFoundation -framework Security
	JAVA_HOME:= $(shell java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | sed 's/\s*java.home = //' | sed 's/\/jre//')
endif

ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
CORE_DIR:=core
BIND_DIR:=bindings
CBIND_DIR:=c
JBIND_DIR:=java
EXAMPLES_DIR:=examples