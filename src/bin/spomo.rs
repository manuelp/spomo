use std::env;
use std::time::{Duration, Instant};

use error_stack::ResultExt;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::init;

fn read_duration() -> AppResult<Duration> {
    let cli_arguments: Vec<_> = env::args().map(|a| a.to_owned()).collect();
    let duration_spec = cli_arguments.get(1).ok_or(AppError)?;
    feature::duration_parsing::parse_duration(duration_spec)
        .change_context(AppError)
        .attach("cannot parse duration spec")
}

fn main() -> AppResult<()> {
    init::error_reporting();
    init::tracing();

    let duration_spec = read_duration()?;

    let started = Instant::now();
    let end_time = started + duration_spec;
    loop {
        let now = Instant::now();
        let elapsed_time = now.duration_since(started);
        let remaining_time = end_time.duration_since(now);
        println!("Remaining: {:?}, elapsed: {:?}", remaining_time, elapsed_time);
        std::thread::sleep(Duration::from_secs(1));
        if elapsed_time > duration_spec {
            break;
        }
    }
    println!("DING!");

    Ok(())
}
