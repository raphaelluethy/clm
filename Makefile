.PHONY: all build install check clean uninstall

# Default target
all: build

# Build the binary in release mode
build:
	cargo build --release

# Install the binary to /usr/local/bin
install:
	cargo install --path .
	@echo "clm installed to cargo bin directory (see 'cargo bin')"

# Run typechecking
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean

# Uninstall the binary
uninstall:
	cargo uninstall clm || true
	@echo "clm uninstalled from cargo bin directory"
