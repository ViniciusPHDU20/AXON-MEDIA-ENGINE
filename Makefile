# AXON MEDIA ENGINE v7.2 - Makefile Professional
# Optimized for Arch Linux / CachyOS

PROJECT_NAME = axon-media-engine
BINARY = target/release/$(PROJECT_NAME)

.PHONY: all setup build run clean windows help

all: setup build

help:
	@echo "AXON MEDIA ENGINE - Command Hub"
	@echo "  setup      Install system dependencies (pacman)"
	@echo "  build      Compile the project in release mode (Linux)"
	@echo "  windows    Cross-compile for Windows (.exe)"
	@echo "  run        Compile and execute the engine"
	@echo "  clean      Remove build artifacts"

setup:
	@echo "[*] Installing dependencies for $(PROJECT_NAME)..."
	sudo pacman -S --needed --noconfirm mpv rustup gcc pkg-config nvidia-utils mingw-w64-gcc

build:
	@echo "[*] Building release binary..."
	cargo build --release

windows:
	@chmod +x build_windows.sh
	./build_windows.sh

run: build
	@echo "[*] Launching AXON MEDIA ENGINE..."
	./$(BINARY)

clean:
	@echo "[*] Cleaning project..."
	cargo clean

check:
	@echo "[*] Checking code integrity..."
	cargo check
