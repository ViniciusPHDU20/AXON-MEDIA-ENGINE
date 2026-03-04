# 🚀 AXON MEDIA ENGINE v7.2

**AXON MEDIA ENGINE** is a high-performance, professional-grade media playback solution built with **Rust**. It combines the raw power of `libmpv` with a modern, reactive interface powered by `egui`, specifically optimized for Linux environments utilizing NVIDIA hardware and Wayland.

## 🛠 Core Features

- **Next-Gen Playback Engine:** Leveraging `libmpv2` for robust, frame-perfect media decoding.
- **Hardware Acceleration:** Native support for **NVDEC (NVIDIA Video Decoder)** via `gpu-next` and `nvdec` profiles.
- **Real-Time Telemetry:** Integrated hardware monitoring (GPU and CPU load) using NVML and system-level hooks.
- **Optimized for Wayland:** Dedicated configurations for seamless operation on modern Linux display servers.
- **Minimalist Interface:** A clean, high-performance UI designed for low latency and maximum focus.

## 🧰 Tech Stack

| Component | Technology |
| :--- | :--- |
| **Language** | Rust (2021 Edition) |
| **UI Framework** | [egui / eframe](https://github.com/emilk/egui) |
| **Media Backend** | [libmpv2](https://github.com/Paradox0/libmpv2-rs) |
| **Hardware Stats** | [NVML (nvml-wrapper)](https://github.com/Screwtapello/nvml-wrapper) & [sysinfo](https://github.com/GuillaumeGomez/sysinfo) |
| **Error Handling** | [anyhow](https://github.com/dtolnay/anyhow) |

## 🚀 Getting Started

### Windows Specifics (libmpv)

1. Run `.\scripts\update_mpv.bat` to download the developer package from GitHub.
2. Extract the contents to the project root.
3. Keep the name **`libmpv-2.dll`** as is.
4. Rename the import library **`libmpv.dll.a`** to **`libmpv-2.lib`** in the root folder.
5. Compile using `cargo build --release`.

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ViniciusPHDU20/AXON-MEDIA-ENGINE.git
   cd AXON-MEDIA-ENGINE
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the engine:**
   ```bash
   cargo run --release
   ```

## 📈 Architecture Note

The v7.2 release marks a significant architectural shift:
- **Migration to libmpv2:** Removed legacy FFmpeg manual decoding for better stability and feature support.
- **Telemetry Thread:** Decoupled hardware monitoring from the UI thread to ensure consistent 60+ FPS in the interface.
- **Wayland Optimization:** Explicit `gpu-context=wayland` setting to prevent flickering and performance regressions on modern desktops.

## 🛡️ License

This project is licensed under the MIT License - see the LICENSE file for details.

---
*Developed by ViniciusPHDU20*
