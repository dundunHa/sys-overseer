use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sysinfo::Networks;

#[derive(Debug, Clone)]
struct NetworkStats {
    received_bytes: u64,
    transmitted_bytes: u64,
    timestamp: Instant,
}

#[derive(Default)]
pub struct NetworkMonitor {
    networks: Networks,
    previous_stats: Arc<Mutex<HashMap<String, NetworkStats>>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        NetworkMonitor {
            networks: Networks::new_with_refreshed_list(),
            previous_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_network_info(&mut self) -> (u64, u64) {
        self.networks.refresh(true);

        let mut total_received = 0u64;
        let mut total_transmitted = 0u64;

        let mut previous_stats = self.previous_stats.lock().unwrap();
        let now = Instant::now();

        for (interface_name, data) in self.networks.iter() {
            let current_stats = NetworkStats {
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

                    total_received += received_speed;
                    total_transmitted += transmitted_speed;
                }
            }

            previous_stats.insert(interface_name.to_string(), current_stats);
        }

        (total_received, total_transmitted)
    }

    pub fn get_interface_names(&self) -> Vec<String> {
        self.networks
            .iter()
            .map(|(name, _)| name.to_string())
            .collect()
    }

    pub fn get_interface_info(&mut self, interface_name: &str) -> Option<(u64, u64)> {
        self.networks.refresh(true);

        let data = self.networks.get(interface_name)?;
        let mut previous_stats = self.previous_stats.lock().unwrap();
        let now = Instant::now();

        let current_stats = NetworkStats {
            received_bytes: data.total_received(),
            transmitted_bytes: data.total_transmitted(),
            timestamp: now,
        };

        let result = if let Some(prev_stats) = previous_stats.get(interface_name) {
            let time_diff = current_stats
                .timestamp
                .duration_since(prev_stats.timestamp)
                .as_secs_f64();

            if time_diff > 0.0 {
                let received_speed = ((current_stats.received_bytes - prev_stats.received_bytes)
                    as f64
                    / time_diff) as u64;
                let transmitted_speed = ((current_stats.transmitted_bytes
                    - prev_stats.transmitted_bytes) as f64
                    / time_diff) as u64;
                Some((received_speed, transmitted_speed))
            } else {
                Some((0, 0))
            }
        } else {
            Some((0, 0))
        };

        previous_stats.insert(interface_name.to_string(), current_stats);
        result
    }
}
