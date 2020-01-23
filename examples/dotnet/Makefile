SHELL := /bin/sh

include ../../config.mk

.PHONY : dotnet-examples clean

dotnet-examples:
	dotnet build Example.csproj -o $(OUT_DIR)/dotnet

clean:
	rm -rf $(OUT_DIR)/dotnet