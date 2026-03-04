#!/bin/bash
# AXON MEDIA ENGINE - Windows Cross-Compiler Helper
# Requires: mingw-w64-gcc

echo "[*] Starting Cross-Compilation for Windows (x86_64)..."

# Configurações para o Cargo usar o linker do MinGW
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

# Tenta compilar
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "----------------------------------------------------"
    echo "[SUCCESS] Executable created at:"
    echo "target/x86_64-pc-windows-gnu/release/axon-media-engine.exe"
    echo "----------------------------------------------------"
    echo "[TIP] Remember to place 'mpv.dll' in the same folder as the .exe on Windows!"
else
    echo "[ERROR] Compilation failed. Check if mingw-w64 is correctly installed."
fi
