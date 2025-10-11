# ─────────────────────────────
# Makefile for nanofetch
# ─────────────────────────────

# Compiler
CC = gcc

# Source and target
SRC = nanofetch.c
TARGET = nanofetch 

# Compiler flags for max optimization
CFLAGS = -O3 -march=native -flto -funroll-loops -ffast-math -DNDEBUG

# Default target
all: $(TARGET)

# Build executable
$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET)
	chmod +x $(TARGET)

# Move to ~/.local/bin (create folder if not exists)
setup: $(TARGET)
	mkdir -p $(HOME)/.local/bin
	cp $(TARGET) $(HOME)/.local/bin/
	chmod +x $(HOME)/.local/bin/$(TARGET)
	@echo "Installed $(TARGET) to $(HOME)/.local/bin. Make sure it's in your PATH!"

# Clean build files
clean:
	rm -f $(TARGET)
