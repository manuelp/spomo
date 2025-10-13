use chrono::Local;
use error_stack::ResultExt;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::feature::audio::{Beeper, SimpleBeeper};
use spomo::init;
use spomo::common::format_time;
use owo_colors::OwoColorize;
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

fn ding() -> AppResult<()> {
    println!("{}", "DING!".cyan());
    SimpleBeeper::default()
        .beep()
        .change_context(AppError)
        .attach("cannot reproduce beep")
}

fn main() -> AppResult<()> {
    init::error_reporting();
    init::tracing();

    let duration_spec = read_duration()?;
    let duration_secs = duration_spec.as_secs();

    let started = Instant::now();
    loop {
        let now = Instant::now();
        let elapsed_secs = (now - started).as_secs();
        let remaining_secs = duration_secs - elapsed_secs;
        println!(
            "Remaining: {}\telapsed: {}",
            format_time(remaining_secs).red(),
            format_time(elapsed_secs).green()
        );
        thread::sleep(Duration::from_secs(1));
        if elapsed_secs >= duration_secs {
            break;
        }
    }
    ding()?;
    println!("--------------------");
    println!("Ended: {}", Local::now().to_rfc3339());

    Ok(())
}
