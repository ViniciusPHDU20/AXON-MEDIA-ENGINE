use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::System;
use nvml_wrapper::Nvml;

pub struct HardwareStats {
    pub gpu_load: u32,
    pub cpu_load: u32,
    pub gpu_temp: u32,
    pub mem_used: u64,
}

pub fn spawn_telemetry_thread(stats: Arc<Mutex<HardwareStats>>) {
    thread::spawn(move || {
        let nvml = Nvml::init().ok();
        let mut sys = System::new_all();
        
        loop {
            sys.refresh_cpu();
            sys.refresh_memory();
            
            let (g_load, g_temp) = if let Some(ref n) = nvml {
                if let Ok(device) = n.device_by_index(0) {
                    let load = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                    let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap_or(0);
                    (load, temp)
                } else {
                    (0, 0)
                }
            } else {
                // Fallback Linux (GPU Load)
                let load = std::fs::read_to_string("/sys/class/drm/card0/device/gpu_busy_percent")
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(0);
                (load, 0)
            };

            if let Ok(mut s) = stats.lock() {
                s.gpu_load = g_load;
                s.gpu_temp = g_temp;
                s.cpu_load = sys.global_cpu_info().cpu_usage() as u32;
                s.mem_used = sys.used_memory() / 1024 / 1024; // MB
            }
            
            thread::sleep(Duration::from_millis(1000));
        }
    });
}
