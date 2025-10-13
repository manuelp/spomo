use chrono::{TimeDelta, prelude::*};
use error_stack::ResultExt;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::feature::audio::{Beeper, SimpleBeeper};
use spomo::init;
use std::time::Duration;
use std::{env, thread};

fn read_duration() -> AppResult<Duration> {
    let cli_arguments: Vec<_> = env::args().map(|a| a.to_owned()).collect();
    let duration_spec = cli_arguments.get(1).ok_or(AppError)?;
    feature::duration_parsing::parse_duration(duration_spec)
        .change_context(AppError)
        .attach("cannot parse duration spec")
}

fn ding() -> AppResult<()> {
    println!("DING!");
    SimpleBeeper::default()
        .beep()
        .change_context(AppError)
        .attach("cannot reproduce beep")
}

fn format_time(time: &TimeDelta) -> String {
    format!("{:02}:{:02}", time.num_minutes(), time.num_seconds())
}

fn main() -> AppResult<()> {
    init::error_reporting();
    init::tracing();

    let duration_spec = read_duration()?;
    let duration_spec = TimeDelta::from_std(duration_spec)
        .change_context(AppError)
        .attach("invalid duration spec")?;

    let started = Utc::now();
    let end_time = started + duration_spec;
    loop {
        let now = Utc::now();
        let elapsed_time = now - started;
        let remaining_time = end_time - now;
        println!(
            "Remaining: {}\telapsed: {}",
            format_time(&remaining_time),
            format_time(&elapsed_time)
        );
        thread::sleep(Duration::from_secs(1));
        if elapsed_time > duration_spec {
            break;
        }
    }
    ding()?;

    Ok(())
}
