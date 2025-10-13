use error_stack::ResultExt;
use spomo::error::{AppError, AppResult};
use spomo::feature;
use spomo::init;
use std::fs::File;
use std::time::{Duration, Instant};
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

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .change_context(AppError)
        .attach("cannot open audio output")?;
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    sink.set_volume(0.5);
    sink.append(rodio::source::SineWave::new(932.));
    thread::sleep(Duration::from_secs(1));
    sink.stop();

    Ok(())
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
        println!(
            "Remaining: {:?}, elapsed: {:?}",
            remaining_time, elapsed_time
        );
        thread::sleep(Duration::from_secs(1));
        if elapsed_time > duration_spec {
            break;
        }
    }
    ding()?;

    Ok(())
}
