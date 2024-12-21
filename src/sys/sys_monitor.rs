use lazy_static::lazy_static;
use std::sync::Mutex;
use sysinfo::System;

#[derive(Default, Debug)]
pub struct SystemSnapshot {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
}

lazy_static! {
    static ref SYSTEM: Mutex<System> = Mutex::new(System::new_all());
}

pub fn get_system_snapshot() -> SystemSnapshot {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_cpu_usage();
    let cpu_usage = sys.global_cpu_usage();
    SystemSnapshot {
        cpu_usage,
        ..Default::default()
    }
}