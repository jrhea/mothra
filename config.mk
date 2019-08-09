OS:=$(shell uname -s | tr '[:upper:]' '[:lower:]')

ifeq ($(OS), linux)
	EXT:=so
	OS_LFLAGS:=
else ifeq ($(OS), darwin)
	EXT:=dylib
	OS_LFLAGS:=-mmacosx-version-min=$(shell defaults read loginwindow SystemVersionStampAsString) -framework CoreFoundation -framework Security
endif

ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
CORE_DIR:=core
BIND_DIR:=bindings
CBIND_DIR:=c
JBIND_DIR:=java
EXAMPLES_DIR:=examples