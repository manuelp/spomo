use chrono::Local;
use error_stack::ResultExt;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use spomo::common::format_time;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::feature::audio::{Beeper, SimpleBeeper};
use spomo::init;
use std::time::{Duration, Instant};
use std::{env, thread};

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
        let title = Line::from("spomo".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let remaining_line = Line::from(vec![
            "Remaining:".into(),
            format_time(self.cursor.remaining_secs).red().bold()
        ]);
        let elapsed_line = Line::from(vec!(
            "Elapsed:".into(),
            format_time(self.cursor.elapsed_secs).green(),
        ));
        let time_text = Text::from(vec![remaining_line, elapsed_line]);

        Paragraph::new(time_text)
            .centered()
            .block(block)
            .render(area, buf);
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
    println!("Ended: {}", Local::now().to_rfc3339());
    result
}
