#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod telemetry;

use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rfd::FileDialog;
use libmpv2::Mpv;
use telemetry::{HardwareStats, spawn_telemetry_thread};

struct VideoMetadata {
    duration: f64,
    position: f64,
    filename: String,
}

struct AxonCentral {
    stats: Arc<Mutex<HardwareStats>>,
    mpv: Option<Mpv>,
    is_playing: bool,
    metadata: VideoMetadata,
    error_log: Vec<String>,
}

impl AxonCentral {
    fn new(_cc: &eframe::CreationContext<'_>, stats: Arc<Mutex<HardwareStats>>) -> Self {
        let mut visual = egui::Visuals::dark();
        visual.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(10, 10, 12);
        visual.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 120, 255));
        _cc.egui_ctx.set_visuals(visual);

        let mut error_log = Vec::new();
        let mpv = match Mpv::new() {
            Ok(m) => {
                let _ = m.set_property("vo", "gpu-next");
                let _ = m.set_property("gpu-context", "auto");
                let _ = m.set_property("hwdec", "auto");
                Some(m)
            },
            Err(e) => {
                error_log.push(format!("CRITICAL: libmpv init failed: {}", e));
                None
            }
        };
        
        Self {
            stats,
            mpv,
            is_playing: false,
            metadata: VideoMetadata { 
                duration: 0.0, 
                position: 0.0,
                filename: "Nenhum arquivo carregado".to_string(),
            },
            error_log,
        }
    }

    fn play_video(&mut self, path: &str) {
        if let Some(ref m) = self.mpv {
            if let Ok(_) = m.command("loadfile", &[path, "replace"]) {
                self.is_playing = true;
                self.metadata.filename = std::path::Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
            }
        }
    }

    fn update_metadata(&mut self) {
        if self.is_playing {
            if let Some(ref m) = self.mpv {
                self.metadata.duration = m.get_property("duration").unwrap_or(0.0);
                self.metadata.position = m.get_property("time-pos").unwrap_or(0.0);
            }
        }
    }

    fn seek(&mut self, pos: f64) {
        if let Some(ref m) = self.mpv {
            let _ = m.set_property("time-pos", pos);
        }
    }
}

impl eframe::App for AxonCentral {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metadata();
        
        let stats = {
            let s = self.stats.lock().unwrap();
            (s.gpu_load, s.cpu_load, s.gpu_temp, s.mem_used)
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.heading(egui::RichText::new("AXON MEDIA ENGINE v7.2.1-PRO")
                    .color(egui::Color32::from_rgb(0, 150, 255))
                    .strong());
                ui.label(egui::RichText::new(&self.metadata.filename).italics().size(11.0));
                ui.separator();

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add_space(ctx.available_rect().width() / 4.0);
                    if ui.add(egui::Button::new("📂 LOAD").min_size(egui::vec2(80.0, 28.0))).clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.play_video(&path.display().to_string());
                        }
                    }
                    if self.is_playing {
                        if let Some(ref m) = self.mpv {
                            let paused: bool = m.get_property("pause").unwrap_or(false);
                            let label = if paused { "▶ PLAY" } else { "⏸ PAUSE" };
                            if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 28.0))).clicked() {
                                let _ = m.set_property("pause", !paused);
                            }
                        }
                    }
                });

                ui.add_space(15.0);

                if self.is_playing && self.metadata.duration > 0.0 {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:.0}s", self.metadata.position));
                            let mut seek_pos = self.metadata.position;
                            let slider = egui::Slider::new(&mut seek_pos, 0.0..=self.metadata.duration)
                                .show_value(false)
                                .trailing_fill(true);
                            if ui.add(slider).changed() {
                                self.seek(seek_pos);
                            }
                            ui.label(format!("{:.0}s", self.metadata.duration));
                        });
                    });
                }

                ui.add_space(20.0);

                ui.columns(2, |cols| {
                    cols[0].group(|ui| {
                        ui.label(egui::RichText::new("HARDWARE TELEMETRY").strong().size(10.0));
                        ui.add_space(4.0);
                        ui.label(format!("GPU Load: {}%", stats.0));
                        ui.add(egui::ProgressBar::new(stats.0 as f32 / 100.0).fill(egui::Color32::from_rgb(0, 200, 100)));
                        ui.label(format!("GPU Temp: {}°C", stats.2));
                    });
                    cols[1].group(|ui| {
                        ui.label(egui::RichText::new("SYSTEM RESOURCES").strong().size(10.0));
                        ui.add_space(4.0);
                        ui.label(format!("CPU Load: {}%", stats.1));
                        ui.add(egui::ProgressBar::new(stats.1 as f32 / 100.0).fill(egui::Color32::from_rgb(255, 120, 0)));
                        ui.label(format!("RAM Used: {} MB", stats.3));
                    });
                });

                if !self.error_log.is_empty() {
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("SYSTEM LOGS").color(egui::Color32::RED).strong());
                        for err in &self.error_log {
                            ui.label(egui::RichText::new(err).size(9.0).color(egui::Color32::LIGHT_RED));
                        }
                    });
                }
            });
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> anyhow::Result<()> {
    let stats = Arc::new(Mutex::new(HardwareStats { 
        gpu_load: 0, 
        cpu_load: 0, 
        gpu_temp: 0, 
        mem_used: 0 
    }));
    
    spawn_telemetry_thread(Arc::clone(&stats));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 420.0])
            .with_title("AXON CONTROL PANEL - PROFESSIONAL"),
        ..Default::default()
    };

    eframe::run_native(
        "AXON MEDIA ENGINE",
        options,
        Box::new(|cc| Box::new(AxonCentral::new(cc, stats))),
    ).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}
