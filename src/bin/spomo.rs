use chrono::prelude::*;
use error_stack::ResultExt;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
};
use spomo::common::format_time;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::feature::audio::{Beeper, SimpleBeeper};
use spomo::init;
use std::time::{Duration, Instant};
use std::{env, thread};

const APP_NAME: &'static str = "spomo";

fn read_duration() -> AppResult<Duration> {
    let duration_specs: Vec<_> = env::args().skip(1).collect();
    let mut duration_spec = Duration::ZERO;
    for spec in duration_specs {
        let d = feature::duration_parsing::parse_duration(&spec)
            .change_context(AppError)
            .attach("cannot parse duration spec")?;
        duration_spec += d;
    }
    Ok(duration_spec)
}

#[derive(Debug, Default)]
struct TimeCursor {
    elapsed_secs: u64,
    remaining_secs: u64,
}

#[derive(Debug)]
struct App {
    started: Instant,
    duration_secs: u64,
    cursor: TimeCursor,
}

impl App {
    fn new(duration_secs: u64) -> Self {
        Self {
            started: Instant::now(),
            duration_secs,
            cursor: TimeCursor::default(),
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let elapsed_secs = (now - self.started).as_secs();
        let remaining_secs = self.duration_secs - elapsed_secs;
        let new_cursor = TimeCursor {
            elapsed_secs,
            remaining_secs,
        };
        self.cursor = new_cursor;
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult<()> {
        loop {
            self.tick();

            terminal
                .draw(|frame| self.draw(frame))
                .change_context(AppError)
                .attach("cannot render frame")?;

            thread::sleep(Duration::from_secs(1));
            if self.cursor.elapsed_secs >= self.duration_secs {
                break;
            }
        }

        SimpleBeeper::default()
            .beep()
            .change_context(AppError)
            .attach("cannot reproduce beep")?;

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main content block
        let title = Line::from(APP_NAME.bold());
        let main_block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        // Keybindings block
        let keybindings_hint = Span::styled("(q) Quit", Style::default().fg(Color::LightYellow));
        let keybindings_block = Paragraph::new(Line::from(keybindings_hint))
            .block(Block::bordered().borders(Borders::ALL))
            .centered();

        // Outer layout
        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);
        let main_chunk = outer_chunks[0];
        let footer_chunk = outer_chunks[1];

        // Render the main blocks
        main_block.render(main_chunk, buf);
        keybindings_block.render(footer_chunk, buf);

        // ------ Main block content (layout and rendering)
        // Time text
        let remaining_text = format_time(self.cursor.remaining_secs);
        let remaining_line = Line::from(vec![
            "Remaining:".into(),
            remaining_text.clone().red().bold(),
        ]);
        let elapsed_text = format_time(self.cursor.elapsed_secs);
        let elapsed_line = Line::from(vec!["Elapsed:".into(), elapsed_text.clone().green()]);
        let time_text = Text::from(vec![remaining_line, elapsed_line]);

        // Layout work to centering vertically the time text and gauge
        let text_height = time_text.height() as u16;
        let total_height = text_height + 1; // text + gauge + footer;

        let empty_space = main_chunk.height.saturating_sub(total_height);
        let padding = empty_space / 2;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(padding),
                Constraint::Length(text_height),
                Constraint::Length(1), // gauge
                Constraint::Length(padding)
            ])
            .margin(1)
            .split(area);

        // Render the paragraph in the middle rect produced by the vertical layout
        Paragraph::new(time_text)
            .alignment(Alignment::Center)
            .render(chunks[1], buf);

        let ratio = self.cursor.remaining_secs as f64 / self.duration_secs as f64;
        let label = Span::styled(
            remaining_text,
            Style::default().italic().bold().fg(Color::DarkGray),
        );
        Gauge::default()
            .ratio(ratio)
            .label(label)
            .gauge_style(Style::default().bg(Color::Red).fg(Color::Green))
            .render(chunks[2], buf);
    }
}

fn main() -> AppResult<()> {
    init::error_reporting();
    init::tracing();
    let mut terminal = ratatui::init();

    let duration_spec = read_duration()?;
    let duration_secs = duration_spec.as_secs();
    let mut app = App::new(duration_secs);
    let result = app.run(&mut terminal);

    ratatui::restore();
    println!("Ended: {}", Local::now());
    result
}
