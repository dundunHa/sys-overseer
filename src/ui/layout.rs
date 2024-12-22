use crate::sys::sys_monitor::get_system_snapshot;
use crate::ui::widgets::ChartState;
use core::f64;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Default)]
struct AppLayout {
    top_left: Rect,
    top_right: Rect,
    bottom: Rect,
}

impl AppLayout {
    fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(15)])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]) 
            .split(chunks[0]);

        AppLayout {
            top_left: top_chunks[0],
            top_right: top_chunks[1],
            bottom: chunks[1],
        }
    }
}

struct App {
    chart_state: ChartState,
    layout: AppLayout,
}

impl App {
    pub fn new() -> Self {
        Self {
            chart_state: ChartState::new(100),
            layout: AppLayout::default(),
        }
    }

    pub fn update(&mut self, cpu_usage: f64, memory_used: f64, memory_total: f64) {
        self.chart_state.add_point(cpu_usage);
        self.chart_state.cpu_usage = cpu_usage;
        self.chart_state.memory_used = memory_used;
        self.chart_state.memory_total = memory_total;
    }

    pub fn ui<'a>(&'a mut self, frame: &mut Frame) {
        self.layout = AppLayout::new(frame.area());

        let chart = self.chart_state.render();
        frame.render_widget(chart, self.layout.top_left);

        let sys_info = self.chart_state.render_sys_info();
        frame.render_widget(sys_info, self.layout.top_right);

        let bottom_text = self.chart_state.render_botom_bar();
        frame.render_widget(bottom_text, self.layout.bottom);
    }
}

struct TerminalManager {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalManager {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal =
            Terminal::new(backend).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Self { terminal })
    }

    fn get_terminal(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        let mut cleanup_operations = || -> io::Result<()> {
            disable_raw_mode()?;
            execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )
        };

        if let Err(e) = cleanup_operations() {
            eprintln!("Error while cleaning up terminal: {}", e);
        }
    }
}

pub fn run_app() -> io::Result<()> {
    let mut terminal_manager = TerminalManager::new()?;
    let mut app = App::new();

    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(1);

    loop {
        let now = Instant::now();
        let time_until_next_update =
            if let Some(wait_time) = update_interval.checked_sub(now.duration_since(last_update)) {
                wait_time
            } else {
                Duration::from_secs(0)
            };

        if event::poll(time_until_next_update)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if now.duration_since(last_update) >= update_interval {
            let sys_info = get_system_snapshot();
            app.update(
                sys_info.cpu_usage as f64,
                sys_info.memory_used as f64,
                sys_info.memory_total as f64,
            );

            terminal_manager.get_terminal().draw(|frame| {
                app.ui(frame);
            })?;

            last_update = now;
        }
    }

    Ok(())
}
