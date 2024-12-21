use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};


pub struct ChartState {
    data_points: Vec<(f64, f64)>,
    window_size: usize,
    current_x: f64, 
}

impl ChartState {
    pub fn new(window_size: usize) -> Self {
        Self {
            data_points: Vec::new(),
            window_size,
            current_x: 0.0,
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

    pub fn render<'a>(&'a self) -> Chart<'a> {
        let datasets = vec![Dataset::default()
            .name("CPU Usage")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)  // 改用线图可能更适合展示 CPU 使用率
            .style(Style::default().fg(Color::Magenta))
            .data(&self.data_points)];

        let x_bounds = if !self.data_points.is_empty() {
            [0.0, self.window_size as f64]
        } else {
            [0.0, self.window_size as f64]
        };

        Chart::new(datasets)
            .block(Block::default().title("CPU Usage").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .bounds(x_bounds)
                    .labels(vec![
                        format!("{:.0}", x_bounds[0]),
                        format!("{:.0}", x_bounds[1] / 2.0),
                        format!("{:.0}", x_bounds[1])
                    ])
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(vec!["0".to_string(), "50".to_string(), "100".to_string()])
            )
    }
}
