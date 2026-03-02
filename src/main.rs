use eframe::egui;
use std::process::{Command, Stdio, Child};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{Write, BufReader, Read};
use sysinfo::System;
use rfd::FileDialog;
use nvml_wrapper::Nvml;

// Cross-Platform IPC
use interprocess::local_socket::{LocalSocketStream, NameTypeSupport};

struct HardwareStats {
    gpu_load: u32,
    gpu_temp: u32,
    cpu_load: u32,
    ram_gb: f32,
}

struct AxonCentral {
    stats: Arc<Mutex<HardwareStats>>,
    playlist: Vec<String>,
    mpv_child: Option<Child>,
    ipc_stream: Option<LocalSocketStream>,
    socket_name: String,
}

impl AxonCentral {
    fn new(_cc: &eframe::CreationContext<'_>, stats: Arc<Mutex<HardwareStats>>) -> Self {
        // Definir nome do Socket/Pipe baseado no OS
        let socket_name = if cfg!(windows) {
            r"\\.\pipe\axon_media_pipe".to_string()
        } else {
            "/tmp/axon.sock".to_string()
        };

        Self {
            stats,
            playlist: Vec::new(),
            mpv_child: None,
            ipc_stream: None,
            socket_name,
        }
    }

    fn play_video(&mut self, path: String) {
        if let Some(mut child) = self.mpv_child.take() { let _ = child.kill(); }

        // Configurações específicas por OS
        let mpv_bin = if cfg!(windows) { "mpv.exe" } else { "mpv" };
        let vo_driver = if cfg!(windows) { "gpu" } else { "gpu-next" }; // Windows é mais estável com 'gpu'
        let gpu_ctx = if cfg!(windows) { "d3d11" } else { "wayland" };
        let hw_dec = if cfg!(windows) { "d3d11va" } else { "nvdec" };

        // Verificar se MPV existe no Windows
        if cfg!(windows) && std::path::Path::new(mpv_bin).exists() == false {
            println!("[!] MPV não encontrado. Execute o script 'Update-Windows.bat'.");
            return;
        }

        let child = Command::new(mpv_bin)
            .arg(format!("--input-ipc-server={}", self.socket_name))
            .arg(format!("--vo={}", vo_driver))
            .arg(format!("--gpu-context={}", gpu_ctx))
            .arg(format!("--hwdec={}", hw_dec))
            .arg("--force-window=immediate")
            .arg("--title=AXON_VIDEO_PLAYER")
            .arg(&path)
            .spawn();

        match child {
            Ok(c) => {
                self.mpv_child = Some(c);
                self.connect_ipc();
            },
            Err(e) => println!("[X] Erro ao iniciar MPV: {}", e),
        }
    }

    fn connect_ipc(&mut self) {
        let name = self.socket_name.clone();
        thread::spawn(move || {
            // Tentar conectar por 5 segundos
            for _ in 0..10 {
                thread::sleep(Duration::from_millis(500));
                if let Ok(mut conn) = LocalSocketStream::connect(name.as_str()) {
                    let _ = conn.write_all(b"{ \"command\": [\"show-text\", \"AXON CONNECTED\"] }\n");
                    break;
                }
            }
        });
    }
}

impl eframe::App for AxonCentral {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let s = self.stats.lock().unwrap();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📁 ABRIR").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        self.play_video(path.display().to_string());
                    }
                }
                if cfg!(windows) && ui.button("⬇ ATUALIZAR ENGINE").clicked() {
                    let _ = Command::new("cmd").args(&["/C", "scripts\\update_mpv.bat"]).spawn();
                }
                ui.separator();
                ui.heading("AXON HYBRID");
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("GPU: {}%", s.gpu_load));
                ui.add(egui::ProgressBar::new(s.gpu_load as f32 / 100.0).desired_width(100.0));
                ui.label(format!("CPU: {}%", s.cpu_load));
                ui.add(egui::ProgressBar::new(s.cpu_load as f32 / 100.0).desired_width(100.0));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if self.mpv_child.is_some() {
                    ui.label("REPRODUZINDO");
                } else {
                    ui.label("Pronto.");
                }
            });
        });
        
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> anyhow::Result<()> {
    let stats = Arc::new(Mutex::new(HardwareStats { gpu_load: 0, gpu_temp: 0, cpu_load: 0, ram_gb: 0.0 }));
    let stats_clone = Arc::clone(&stats);

    thread::spawn(move || {
        let nvml = Nvml::init().ok();
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let mut g_load = 0;
            if let Some(ref n) = nvml {
                if let Ok(d) = n.device_by_index(0) {
                    g_load = d.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                }
            }
            if let Ok(mut s) = stats_clone.lock() {
                s.gpu_load = g_load;
                s.cpu_load = sys.global_cpu_info().cpu_usage() as u32;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "AXON MEDIA ENGINE",
        options,
        Box::new(|cc| Box::new(AxonCentral::new(cc, stats))),
    ).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}
