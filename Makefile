# Makefile for rfetch

# Variables
CARGO = cargo
BINARY_NAME = rfetch
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug
INSTALL_PREFIX = /usr/local
INSTALL_DIR = $(INSTALL_PREFIX)/bin
MAN_DIR = $(INSTALL_PREFIX)/share/man/man1

# Default target
.PHONY: all
all: build

# Build in debug mode
.PHONY: build
build:
	$(CARGO) build

# Build in release mode
.PHONY: release
release:
	$(CARGO) build --release

# Run the application
.PHONY: run
run:
	$(CARGO) run

# Run with specific arguments
.PHONY: run-args
run-args:
	$(CARGO) run -- $(ARGS)

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Run tests with output
.PHONY: test-verbose
test-verbose:
	$(CARGO) test -- --nocapture

# Check code without building
.PHONY: check
check:
	$(CARGO) check

# Format code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Check formatting
.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt -- --check

# Run clippy lints
.PHONY: clippy
clippy:
	$(CARGO) clippy -- -D warnings

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Install the binary
.PHONY: install
install: release
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@mkdir -p $(INSTALL_DIR)
	@cp $(RELEASE_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/
	@chmod 755 $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "$(BINARY_NAME) installed successfully!"

# Uninstall the binary
.PHONY: uninstall
uninstall:
	@echo "Uninstalling $(BINARY_NAME) from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "$(BINARY_NAME) uninstalled successfully!"

# Create a source distribution
.PHONY: dist
dist: clean
	@echo "Creating source distribution..."
	@mkdir -p dist
	@tar --exclude='dist' --exclude='target' --exclude='.git' \
	     -czf dist/$(BINARY_NAME)-src.tar.gz .
	@echo "Source distribution created: dist/$(BINARY_NAME)-src.tar.gz"

# Cross-compile for different targets
.PHONY: cross-compile
cross-compile:
	@echo "Cross-compiling for multiple targets..."
	@mkdir -p dist
	# Linux x86_64
	$(CARGO) build --release --target x86_64-unknown-linux-gnu
	cp $(TARGET_DIR)/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) \
	   dist/$(BINARY_NAME)-x86_64-linux
	# Linux aarch64
	$(CARGO) build --release --target aarch64-unknown-linux-gnu
	cp $(TARGET_DIR)/aarch64-unknown-linux-gnu/release/$(BINARY_NAME) \
	   dist/$(BINARY_NAME)-aarch64-linux
	# macOS x86_64
	$(CARGO) build --release --target x86_64-apple-darwin
	cp $(TARGET_DIR)/x86_64-apple-darwin/release/$(BINARY_NAME) \
	   dist/$(BINARY_NAME)-x86_64-macos
	# macOS aarch64 (Apple Silicon)
	$(CARGO) build --release --target aarch64-apple-darwin
	cp $(TARGET_DIR)/aarch64-apple-darwin/release/$(BINARY_NAME) \
	   dist/$(BINARY_NAME)-aarch64-macos

# Development setup
.PHONY: dev-setup
dev-setup:
	@echo "Setting up development environment..."
	rustup component add rustfmt clippy
	@echo "Development environment setup complete!"

# Run all checks (format, clippy, test)
.PHONY: ci
ci: fmt-check clippy test
	@echo "All CI checks passed!"

# Generate documentation
.PHONY: doc
doc:
	$(CARGO) doc --no-deps --open

# Show binary size
.PHONY: size
size: release
	@echo "Binary size information:"
	@ls -lh $(RELEASE_DIR)/$(BINARY_NAME)
	@echo "Stripped size:"
	@strip $(RELEASE_DIR)/$(BINARY_NAME) -o /tmp/$(BINARY_NAME)-stripped
	@ls -lh /tmp/$(BINARY_NAME)-stripped
	@rm /tmp/$(BINARY_NAME)-stripped

# Show help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build        - Build in debug mode"
	@echo "  release      - Build in release mode"
	@echo "  run          - Run the application"
	@echo "  test         - Run tests"
	@echo "  check        - Check code without building"
	@echo "  fmt          - Format code"
	@echo "  fmt-check    - Check code formatting"
	@echo "  clippy       - Run clippy lints"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install binary to system"
	@echo "  uninstall    - Uninstall binary from system"
	@echo "  dist         - Create source distribution"
	@echo "  cross-compile- Cross-compile for multiple targets"
	@echo "  dev-setup    - Setup development environment"
	@echo "  ci           - Run all CI checks"
	@echo "  doc          - Generate and open documentation"
	@echo "  size         - Show binary size information"
	@echo "  help         - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  ARGS         - Arguments to pass to 'make run-args'"
	@echo "  INSTALL_PREFIX - Installation prefix (default: /usr/local)"

# Benchmark (if criterion is added to dev-dependencies)
.PHONY: bench
bench:
	$(CARGO) bench

# Profile the application
.PHONY: profile
profile: release
	@echo "Running performance profile..."
	time ./$(RELEASE_DIR)/$(BINARY_NAME)
