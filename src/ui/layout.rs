use crate::sys::sys_monitor::get_system_snapshot;
use crate::ui::widgets::ChartState;
use core::f64;
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::{widgets::Dataset, Frame};
use std::{io, thread, time::Duration};
#[derive(Debug, Clone, Default)]
struct AppLayout {
    top_left: Rect,
    top_right: Rect,
    bottom: Rect,
    datasets: Vec<Dataset<'static>>,
}

struct App {
    chart_state: ChartState,
}

impl App {
    pub fn new() -> Self {
        Self {
            chart_state: ChartState::new(100),
        }
    }

    pub fn update(&mut self, cpu_usage: f64) {
        self.chart_state.add_point(cpu_usage);
    }

    pub fn ui<'a>(&'a self, frame: &mut Frame) {
        let chart = self.chart_state.render();
        frame.render_widget(chart, frame.area());
    }
}

impl AppLayout {
    fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        AppLayout {
            top_left: top_chunks[0],
            top_right: top_chunks[1],
            bottom: chunks[1],

            ..Default::default()
        }
    }
}

fn init() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

pub fn run_app() -> Result<(), std::io::Error> {
    let mut terminal = init()?;
    let mut app = App::new();

    loop {
        thread::sleep(Duration::from_millis(300));

        let sys_info = get_system_snapshot();
        app.update(sys_info.cpu_usage as f64);

        terminal.draw(|frame| {
            app.ui(frame);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}
