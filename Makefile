# ─────────────────────────────
# Makefile for nanofetch (Rust)
# ─────────────────────────────

TARGET = nanofetch
RELEASE_BIN = target/release/$(TARGET)

# Default target: compile release binary with cargo
all: $(TARGET)

$(TARGET): $(RELEASE_BIN)
	cp $(RELEASE_BIN) $(TARGET)
	chmod +x $(TARGET)

$(RELEASE_BIN): Cargo.toml src/*.rs
	cargo build --release

# Run unit tests
test:
	cargo test

# Move to ~/.local/bin (create folder if not exists)
setup: $(TARGET)
	mkdir -p $(HOME)/.local/bin
	cp $(TARGET) $(HOME)/.local/bin/
	chmod +x $(HOME)/.local/bin/$(TARGET)
	@echo "Installed $(TARGET) to $(HOME)/.local/bin. Make sure it's in your PATH!"

# Clean build files
clean:
	cargo clean
	rm -f $(TARGET)
