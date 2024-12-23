use ratatui::{
    style::{Color, Modifier, Style},
    symbols,
    text::Text,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
};
use std::default::Default;
use crate::sys::network_stats::NetworkMonitor;

#[derive(Default)]
pub struct ChartState {
    data_points: Vec<(f64, f64)>,
    window_size: usize,
    current_x: f64,
    pub cpu_usage: f64,
    pub memory_used: f64,
    pub memory_total: f64,
    pub network_monitor: NetworkMonitor


}
impl ChartState {
    pub fn new(window_size: usize) -> Self {
        Self {
            data_points: Vec::new(),
            window_size,
            current_x: 0.0,
            network_monitor: NetworkMonitor::new(),
            ..Default::default()
        }
    }

    pub fn add_point(&mut self, y: f64) {
        self.data_points.push((self.current_x, y));
        self.current_x += 1.0;

        if self.data_points.len() > self.window_size {
            self.data_points.remove(0);
            for (i, point) in self.data_points.iter_mut().enumerate() {
                point.0 = i as f64;
            }
            self.current_x = (self.window_size - 1) as f64;
        }
    }

    pub fn render(&self) -> Chart<'_> {
        // 创建数据集
        let dataset = Dataset::default()
            .name("CPU Usage")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&self.data_points);

        let x_bounds = [0.0, self.window_size as f64];

        let y_labels = ["0%", "25%", "50%", "75%", "100%"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title("Usage(%) - Real-time")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .bounds(x_bounds)
                    .labels(vec!["".to_string()]),
            )
            .y_axis(Axis::default().bounds([0.0, 100.0]).labels(y_labels))
    }

    pub fn render_sys_info<'a>(&'a mut self) -> Paragraph<'a> {
        //
        let cpu_info = format!("CPU Usage: {:.1}%", self.cpu_usage);
        let memory_usage = format!(
            "Memory Usage: {:.1}%",
            self.memory_used / self.memory_total * 100.0
        );
        let memory_info = format!("Memory Used: {}", self.format_bytes(self.memory_used));
        let memory_total = format!("Memory Total: {}", self.format_bytes(self.memory_total));

        let (tx,rx,_,_) = self.network_monitor.get_network_info();
        let tx_label = format!("TX: {}/s", self.format_network_speed(tx));
        let rx_label = format!("RX: {}/s", self.format_network_speed(rx));
        let text = Text::styled(
            format!(
                "{cpu_info}\n\n{memory_info}\n{memory_total}\n{memory_usage}\n\n{tx_label}\n{rx_label}",
            ),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        Paragraph::new(text)
            .block(Block::default().title("System Info").borders(Borders::ALL))
            .style(Style::default())
    }

    pub fn render_botom_bar(& self) -> Paragraph {
        let quit_text = "<q> Quit";

        let text = Text::styled(
            quit_text,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        Paragraph::new(text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .style(Style::default())
    }
    fn format_bytes(&self, bytes: f64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let size = bytes as f64;

        if size >= GB {
            format!("{:.2} GB", size / GB)
        } else if size >= MB {
            format!("{:.2} MB", size / MB)
        } else if size >= KB {
            format!("{:.2} KB", size / KB)
        } else {
            format!("{} bytes", bytes)
        }
    }

    pub fn format_network_speed(&self,bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * KB;
        const GB: f64 = KB * KB * KB;
    
        let result = match bytes as f64 {
            bytes if bytes >= GB => {
                format!("{:.2} GB", bytes / GB)
            },
            bytes if bytes >= MB => {
                format!("{:.2} MB", bytes / MB)
            },
            bytes => {
                format!("{:.2} KB", bytes / KB)
            }
        };
    
        result
    }
}
