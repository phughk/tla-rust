# Targets
.PHONY: test install

# Default target
default: install

# Run tests
test:
	cargo test --workspace

# Build in release mode
install:
	cargo build --workspace --release
