use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::System;
use rfd::FileDialog;
use nvml_wrapper::Nvml;
use libmpv2::Mpv;

struct HardwareStats {
    gpu_load: u32,
    cpu_load: u32,
}

struct VideoMetadata {
    duration: f64,
    position: f64,
}

struct AxonCentral {
    stats: Arc<Mutex<HardwareStats>>,
    mpv: Mpv,
    is_playing: bool,
    metadata: VideoMetadata,
}

impl AxonCentral {
    fn new(_cc: &eframe::CreationContext<'_>, stats: Arc<Mutex<HardwareStats>>) -> Self {
        // Estilização Profissional
        let mut visual = egui::Visuals::dark();
        visual.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(15, 15, 20);
        visual.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255));
        _cc.egui_ctx.set_visuals(visual);

        let mpv = Mpv::new().unwrap_or_else(|e| {
            eprintln!("CRITICAL ERROR: Failed to initialize libmpv. Is it installed? Error: {}", e);
            std::process::exit(1);
        });
        
        // Otimização para Wayland + NVIDIA
        let _ = mpv.set_property("vo", "gpu-next");
        let _ = mpv.set_property("gpu-context", "wayland");
        let _ = mpv.set_property("hwdec", "nvdec");
        let _ = mpv.set_property("terminal", "yes");
        
        Self {
            stats,
            mpv,
            is_playing: false,
            metadata: VideoMetadata { duration: 0.0, position: 0.0 },
        }
    }

    fn play_video(&mut self, path: &str) {
        if let Ok(_) = self.mpv.command("loadfile", &[path, "replace"]) {
            self.is_playing = true;
        }
    }

    fn update_metadata(&mut self) {
        if self.is_playing {
            self.metadata.duration = self.mpv.get_property("duration").unwrap_or(0.0);
            self.metadata.position = self.mpv.get_property("time-pos").unwrap_or(0.0);
        }
    }

    fn seek(&mut self, pos: f64) {
        let _ = self.mpv.set_property("time-pos", pos);
    }
}

impl eframe::App for AxonCentral {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metadata();
        
        let (gpu_load, cpu_load) = {
            let s = self.stats.lock().unwrap();
            (s.gpu_load, s.cpu_load)
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("🚀 AXON MEDIA ENGINE v7.2").color(egui::Color32::from_rgb(0, 150, 255)).strong());
                ui.separator();

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add_space(ctx.available_rect().width() / 4.0);
                    if ui.add(egui::Button::new("📁 ABRIR").min_size(egui::vec2(80.0, 30.0))).clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.play_video(&path.display().to_string());
                        }
                    }
                    if self.is_playing {
                        let paused: bool = self.mpv.get_property("pause").unwrap_or(false);
                        let label = if paused { "▶ PLAY" } else { "⏸ PAUSE" };
                        if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 30.0))).clicked() {
                            let _ = self.mpv.set_property("pause", !paused);
                        }
                    }
                });

                ui.add_space(15.0);

                // CONTROLE DE TEMPO (SLIDER)
                if self.is_playing && self.metadata.duration > 0.0 {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new(format!(
                            "Playback: {:.0}s / {:.0}s", 
                            self.metadata.position, 
                            self.metadata.duration
                        )).size(12.0));
                        
                        let mut seek_pos = self.metadata.position;
                        let slider = egui::Slider::new(&mut seek_pos, 0.0..=self.metadata.duration)
                            .show_value(false)
                            .trailing_fill(true);
                        
                        if ui.add(slider).changed() {
                            self.seek(seek_pos);
                        }
                    });
                }

                ui.add_space(20.0);

                // TELEMETRIA
                ui.group(|ui| {
                    ui.label(egui::RichText::new("SYSTEM TELEMETRY").strong().size(10.0));
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("GPU: {}%", gpu_load));
                            ui.add(egui::ProgressBar::new(gpu_load as f32 / 100.0)
                                .desired_width(150.0)
                                .fill(egui::Color32::from_rgb(0, 200, 100)));
                        });
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
                            ui.label(format!("CPU: {}%", cpu_load));
                            ui.add(egui::ProgressBar::new(cpu_load as f32 / 100.0)
                                .desired_width(150.0)
                                .fill(egui::Color32::from_rgb(255, 100, 0)));
                        });
                    });
                });

                ui.add_space(15.0);
                if self.is_playing {
                    ui.colored_label(egui::Color32::from_rgb(0, 150, 255), "Hardware Acceleration: ACTIVE (NVDEC)");
                }
            });
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> anyhow::Result<()> {
    let stats = Arc::new(Mutex::new(HardwareStats { gpu_load: 0, cpu_load: 0 }));
    let stats_clone = Arc::clone(&stats);

    thread::spawn(move || {
        let nvml = Nvml::init().ok();
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let g_load = nvml.as_ref()
                .and_then(|n| n.device_by_index(0).ok())
                .and_then(|d| d.utilization_rates().ok())
                .map(|u| u.gpu)
                .unwrap_or(0);

            if let Ok(mut s) = stats_clone.lock() {
                s.gpu_load = g_load;
                s.cpu_load = sys.global_cpu_info().cpu_usage() as u32;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 350.0])
            .with_title("AXON CONTROL PANEL"),
        ..Default::default()
    };

    eframe::run_native(
        "AXON MEDIA ENGINE",
        options,
        Box::new(|cc| Box::new(AxonCentral::new(cc, stats))),
    ).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}
