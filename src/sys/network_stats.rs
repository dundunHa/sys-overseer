use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sysinfo::Networks;

#[derive(Debug, Clone)]
struct NetworkStats {
    received_bytes: u64,
    transmitted_bytes: u64,
    tx: u64,
    rx: u64,
    timestamp: Instant,
}

#[derive(Default)]
pub struct NetworkMonitor {
    networks: Networks,
    previous_stats: Arc<Mutex<HashMap<String, NetworkStats>>>,
}
const EXCLUDED_INTERFACES: &[&str] = &[
    "lo",     // 本地回环
    "docker", // Docker 网络
    "veth",   // 虚拟接口
    "br-",    // 网桥接口
    "virbr",  // 虚拟网桥
    "vmnet",  // VM 网络接口
];
impl NetworkMonitor {
    pub fn new() -> Self {
        NetworkMonitor {
            networks: Networks::new_with_refreshed_list(),
            previous_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn cleanup_stale_data(&mut self) {
        let mut stats = self.previous_stats.lock().unwrap();
        let current_time = Instant::now();
        let timeout = std::time::Duration::from_secs(60);

        stats.retain(|_, stat| current_time.duration_since(stat.timestamp) < timeout);
    }

    fn is_valid_interface(&self, interface_name: &str) -> bool {
        #[cfg(target_os = "windows")]
        {
            ["ethernet", "以太网", "local area connection", "wi-fi"]
                .iter()
                .any(|&pattern| interface_name.to_lowercase().contains(pattern))
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            ["en", "eth", "ens", "enp"]
                .iter()
                .any(|&pattern| interface_name.starts_with(pattern))
        }
    }

    pub fn get_network_info(&mut self) -> (u64, u64, u64, u64) {
        self.networks.refresh(true);
        self.cleanup_stale_data();
        let mut total_received_byte = 0u64;
        let mut total_transmitted_byte = 0u64;
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;
        let mut previous_stats = self.previous_stats.lock().unwrap();
        let now = Instant::now();

        for (interface_name, data) in self.networks.iter() {
            if !self.is_valid_interface(interface_name) {
                continue;
            }

            let current_stats = NetworkStats {
                tx: data.transmitted(),
                rx: data.received(),
                received_bytes: data.total_received(),
                transmitted_bytes: data.total_transmitted(),
                timestamp: now,
            };

            if let Some(prev_stats) = previous_stats.get(interface_name) {
                let time_diff = current_stats
                    .timestamp
                    .duration_since(prev_stats.timestamp)
                    .as_secs_f64();

                if time_diff > 0.0 {
                    let received_speed = ((current_stats.received_bytes - prev_stats.received_bytes)
                        as f64
                        / time_diff) as u64;
                    let transmitted_speed =
                        ((current_stats.transmitted_bytes - prev_stats.transmitted_bytes) as f64
                            / time_diff) as u64;

                    total_received_byte += received_speed;
                    total_transmitted_byte += transmitted_speed;
                }
                if current_stats.tx > 0 {
                    total_tx += current_stats.tx;
                }
                if current_stats.rx > 0 {
                    total_rx += current_stats.rx;
                }
            }

            previous_stats.insert(interface_name.to_string(), current_stats);
        }

        (
            total_tx,
            total_rx,
            total_received_byte,
            total_transmitted_byte,
        )
    }

    pub fn get_interface_names(&self) -> Vec<String> {
        self.networks
            .iter()
            .map(|(name, _)| name.to_string())
            .collect()
    }

    //     pub fn get_interface_info(&mut self, interface_name: &str) -> Option<(u64, u64)> {
    //         self.networks.refresh(true);

    //         let data = self.networks.get(interface_name)?;
    //         let mut previous_stats = self.previous_stats.lock().unwrap();
    //         let now = Instant::now();

    //         let current_stats = NetworkStats {
    //             received_bytes: data.total_received(),
    //             transmitted_bytes: data.total_transmitted(),
    //             timestamp: now,
    //         };

    //         let result = if let Some(prev_stats) = previous_stats.get(interface_name) {
    //             let time_diff = current_stats
    //                 .timestamp
    //                 .duration_since(prev_stats.timestamp)
    //                 .as_secs_f64();

    //             if time_diff > 0.0 {
    //                 let received_speed = ((current_stats.received_bytes - prev_stats.received_bytes)
    //                     as f64
    //                     / time_diff) as u64;
    //                 let transmitted_speed = ((current_stats.transmitted_bytes
    //                     - prev_stats.transmitted_bytes) as f64
    //                     / time_diff) as u64;
    //                 Some((received_speed, transmitted_speed))
    //             } else {
    //                 Some((0, 0))
    //             }
    //         } else {
    //             Some((0, 0))
    //         };

    //         previous_stats.insert(interface_name.to_string(), current_stats);
    //         result
    //     }
}

impl Drop for NetworkMonitor {
    fn drop(&mut self) {
        if let Ok(mut stats) = self.previous_stats.lock() {
            stats.clear();
        }
    }
}
