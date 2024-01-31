BUILD_DIR := build

default: clox
# Remove all build outputs and intermediate files.
clean:
	@ rm -rf $(BUILD_DIR)
	@ rm -rf gen

# Compile a debug build of clox.
debug:
	@ $(MAKE) -f c.make NAME=cloxd MODE=debug SOURCE_DIR=./
	./build/cloxd

# Compile the C interpreter.
clox:
	@ $(MAKE) -f c.make NAME=clox MODE=release SOURCE_DIR=./
	./build/clox

# Compile the C interpreter as ANSI standard C++.
cpplox:
	@ $(MAKE) -f c.make NAME=cpplox MODE=debug CPP=true SOURCE_DIR=./

