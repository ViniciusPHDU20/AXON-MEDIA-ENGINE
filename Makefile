# AXON MEDIA ENGINE v7.2 - Makefile Professional
# Optimized for Arch Linux / CachyOS

PROJECT_NAME = axon-media-engine
BINARY = target/release/$(PROJECT_NAME)

.PHONY: all setup build run clean update-mpv help

all: setup build

help:
	@echo "AXON MEDIA ENGINE - Command Hub"
	@echo "  setup      Install system dependencies (pacman)"
	@echo "  build      Compile the project in release mode"
	@echo "  run        Compile and execute the engine"
	@echo "  clean      Remove build artifacts"
	@echo "  check      Validate Rust code without building"

setup:
	@echo "[*] Installing dependencies for $(PROJECT_NAME)..."
	sudo pacman -S --needed --noconfirm mpv rustup gcc pkg-config nvidia-utils

build:
	@echo "[*] Building release binary..."
	cargo build --release

run: build
	@echo "[*] Launching AXON MEDIA ENGINE..."
	./$(BINARY)

clean:
	@echo "[*] Cleaning project..."
	cargo clean

check:
	@echo "[*] Checking code integrity..."
	cargo check
