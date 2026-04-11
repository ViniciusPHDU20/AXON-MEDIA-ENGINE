#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod telemetry;

use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::env;
use rfd::FileDialog;
use libmpv2::Mpv;
use telemetry::{HardwareStats, spawn_telemetry_thread};

#[derive(PartialEq, Clone, Copy, Debug)]
enum UpscaleMode {
    None,
    Balanced,
    Cinematic,
    Overlord,
}

struct VideoMetadata {
    duration: f64, position: f64, filename: String,
}

struct AxonCentral {
    stats: Arc<Mutex<HardwareStats>>,
    mpv: Option<Mpv>,
    is_playing: bool,
    metadata: VideoMetadata,
    last_update: Instant,
    playlist: Vec<PathBuf>,
    show_playlist: bool,
    upscale_mode: UpscaleMode,
}

// FORMATADOR DE TEMPO SOBERANO
fn format_time(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

impl AxonCentral {
    fn new(_cc: &eframe::CreationContext<'_>, stats: Arc<Mutex<HardwareStats>>) -> Self {
        let mut style = (*_cc.egui_ctx.style()).clone();
        style.visuals.window_rounding = 12.0.into();
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(120, 0, 255);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 0, 255);
        _cc.egui_ctx.set_style(style);

        let mpv = match Mpv::new() {
            Ok(m) => {
                let _ = m.set_property("vo", "gpu-next");
                let _ = m.set_property("hwdec", "auto");
                let _ = m.set_property("keep-open", "yes");
                let _ = m.set_property("video-sync", "display-resample");
                let _ = m.set_property("hr-seek", "no");
                Some(m)
            },
            Err(_) => None
        };
        
        Self {
            stats, mpv, is_playing: false,
            metadata: VideoMetadata { duration: 0.0, position: 0.0, filename: "SYSTEM SOVEREIGN READY".to_string() },
            last_update: Instant::now(), playlist: Vec::new(), show_playlist: true,
            upscale_mode: UpscaleMode::None,
        }
    }

    fn apply_shaders(&mut self) {
        if let Some(ref m) = self.mpv {
            let b = if cfg!(windows) { "./shaders/" } else { "/home/viniciusphdu/WORKSPACE_CORE/projects/AXON-MEDIA-ENGINE/shaders/" };
            let s_gan = format!("{}Anime4K_Restore_GAN_UUL.glsl", b);
            let s_ul = format!("{}Anime4K_Upscale_CNN_x2_UL.glsl", b);
            let s_m = format!("{}Anime4K_Upscale_CNN_x2_M.glsl", b);

            let shader_str = match self.upscale_mode {
                UpscaleMode::None => "".to_string(),
                UpscaleMode::Balanced => s_m,
                UpscaleMode::Cinematic => format!("{}:{}", s_gan, s_m),
                UpscaleMode::Overlord => format!("{}:{}:{}", s_gan, s_ul, s_ul),
            };

            let _ = m.set_property("glsl-shaders-clr", ""); 
            if !shader_str.is_empty() {
                let _ = m.set_property("glsl-shaders", shader_str);
                let (sh, db) = if self.upscale_mode == UpscaleMode::Overlord { (3.5, "yes") } else { (1.0, "no") };
                let _ = m.set_property("sharpen", sh);
                let _ = m.set_property("deband", db);
            } else {
                let _ = m.set_property("sharpen", 0.0);
            }
        }
    }

    fn play_video(&mut self, path: PathBuf) {
        let path_str = path.display().to_string();
        if let Some(ref m) = self.mpv {
            if let Ok(_) = m.command("loadfile", &[&path_str, "replace"]) {
                let _ = m.set_property("pause", false);
                self.is_playing = true;
                self.metadata.filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if let Some(p) = path.parent() {
                    if let Ok(e) = std::fs::read_dir(p) {
                        self.playlist = e.filter_map(|x| x.ok()).map(|x| x.path())
                            .filter(|x| matches!(x.extension().and_then(|s| s.to_str()).unwrap_or(""), "mp4"|"mkv"|"avi"|"webm")).collect();
                        self.playlist.sort();
                    }
                }
                self.apply_shaders();
            }
        }
    }

    fn handle_hotkeys(&mut self, ctx: &egui::Context) {
        if let Some(ref m) = self.mpv {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::ArrowRight) { let _ = m.command("seek", &["5", "relative"]); }
                if i.key_pressed(egui::Key::ArrowLeft) { let _ = m.command("seek", &["-5", "relative"]); }
                if i.key_pressed(egui::Key::ArrowUp) { let _ = m.command("add", &["volume", "10"]); }
                if i.key_pressed(egui::Key::ArrowDown) { let _ = m.command("add", &["volume", "-10"]); }
                if i.key_pressed(egui::Key::Space) {
                    let p: bool = m.get_property("pause").unwrap_or(false);
                    let _ = m.set_property("pause", !p);
                }
            });
        }
    }

    fn update_metadata(&mut self) {
        if self.is_playing && self.last_update.elapsed() > Duration::from_millis(50) {
            if let Some(ref m) = self.mpv {
                self.metadata.duration = m.get_property("duration").unwrap_or(0.0);
                self.metadata.position = m.get_property("time-pos").unwrap_or(0.0);
                self.last_update = Instant::now();
            }
        }
    }
}

impl eframe::App for AxonCentral {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_hotkeys(ctx);
        self.update_metadata();
        let stats = { let s = self.stats.lock().unwrap(); (s.gpu_load, s.cpu_load, s.gpu_temp, s.mem_used) };

        if self.show_playlist {
            egui::SidePanel::right("playlist").resizable(true).default_width(220.0).show_animated(ctx, self.show_playlist, |ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("NEURAL PLAYLIST").color(egui::Color32::from_rgb(150, 0, 255)).strong());
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for path in self.playlist.clone() {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        if ui.selectable_label(filename == self.metadata.filename, format!("📺 {}", filename)).clicked() { self.play_video(path); }
                    }
                });
            });
        }

        egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::from_rgb(0, 0, 5))).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(15.0);
                    ui.heading(egui::RichText::new("AXON MEDIA ENGINE").color(egui::Color32::from_rgb(0, 180, 255)).strong());
                    ui.label(egui::RichText::new("v12.6.0-GOD").color(egui::Color32::from_rgb(255, 0, 255)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(if self.show_playlist { "❌" } else { "📁" }).clicked() { self.show_playlist = !self.show_playlist; }
                    });
                });
                ui.label(egui::RichText::new(&self.metadata.filename).italics().color(egui::Color32::GRAY));
                ui.separator();

                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.add(egui::Button::new("📂 LOAD SOURCE").min_size(egui::vec2(120.0, 35.0))).clicked() {
                        if let Some(path) = FileDialog::new().pick_file() { self.play_video(path); }
                    }
                    if self.is_playing {
                        if let Some(ref m) = self.mpv {
                            let p: bool = m.get_property("pause").unwrap_or(false);
                            if ui.add(egui::Button::new(if p { "▶ RESUME" } else { "⏸ PAUSE" }).min_size(egui::vec2(100.0, 35.0))).clicked() { let _ = m.set_property("pause", !p); }
                        }
                    }
                    ui.separator();
                    ui.label("SPEED:");
                    let mut speed: f64 = self.mpv.as_ref().and_then(|m| m.get_property("speed").ok()).unwrap_or(1.0);
                    if ui.add(egui::Slider::new(&mut speed, 0.5..=4.0).step_by(0.1).show_value(true)).changed() {
                        if let Some(ref m) = self.mpv { let _ = m.set_property("speed", speed); }
                    }
                });

                ui.add_space(20.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("POWER LEVEL SELECTOR").color(egui::Color32::from_rgb(0, 255, 150)));
                    ui.horizontal(|ui| {
                        let modes = [(UpscaleMode::None, "OFF", egui::Color32::GRAY), (UpscaleMode::Balanced, "BALANCED", egui::Color32::from_rgb(0, 180, 255)), (UpscaleMode::Cinematic, "CINEMATIC", egui::Color32::from_rgb(0, 255, 150)), (UpscaleMode::Overlord, "OVERLORD (GOD)", egui::Color32::from_rgb(255, 0, 255))];
                        for (mode, label, color) in modes {
                            let is_active = self.upscale_mode == mode;
                            if ui.add(egui::Button::new(egui::RichText::new(label).color(if is_active { egui::Color32::BLACK } else { color })).fill(if is_active { color } else { egui::Color32::TRANSPARENT })).clicked() { self.upscale_mode = mode; self.apply_shaders(); }
                        }
                    });
                });

                ui.add_space(20.0);
                if self.is_playing && self.metadata.duration > 0.0 {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format_time(self.metadata.position)).strong().color(egui::Color32::WHITE));
                            let mut pos = self.metadata.position;
                            if ui.add(egui::Slider::new(&mut pos, 0.0..=self.metadata.duration).show_value(false).trailing_fill(true)).changed() {
                                if let Some(ref m) = self.mpv { let _ = m.set_property("time-pos", pos); }
                            }
                            ui.label(egui::RichText::new(format_time(self.metadata.duration)).color(egui::Color32::GRAY));
                        });
                    });
                }

                ui.add_space(20.0);
                ui.columns(2, |cols| {
                    cols[0].group(|ui| {
                        ui.label(egui::RichText::new("RTX 3060 Ti").color(egui::Color32::from_rgb(0, 255, 100)));
                        ui.add(egui::ProgressBar::new(stats.0 as f32 / 100.0).text(format!("GPU: {}% ({}°C)", stats.0, stats.2)).fill(egui::Color32::from_rgb(0, 200, 100)));
                    });
                    cols[1].group(|ui| {
                        ui.label(egui::RichText::new("CPU MULTI-THREAD").color(egui::Color32::from_rgb(255, 120, 0)));
                        ui.add(egui::ProgressBar::new(stats.1 as f32 / 100.0).text(format!("CPU: {}% | RAM: {}MB", stats.1, stats.3)).fill(egui::Color32::from_rgb(255, 120, 0)));
                    });
                });
            });
        });
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

fn main() -> anyhow::Result<()> {
    env::set_var("LC_NUMERIC", "C");
    let stats = Arc::new(Mutex::new(HardwareStats { gpu_load: 0, cpu_load: 0, gpu_temp: 0, mem_used: 0 }));
    spawn_telemetry_thread(Arc::clone(&stats));
    let o = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 580.0]), ..Default::default() };
    eframe::run_native("AXON MEDIA ENGINE", o, Box::new(|cc| Box::new(AxonCentral::new(cc, stats))))
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(())
}
