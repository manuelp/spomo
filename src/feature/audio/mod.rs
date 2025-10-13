use error_stack::{Report, ResultExt};
use std::{thread, time::Duration};

#[derive(Debug, thiserror::Error)]
#[error("an audio error occured")]
pub struct AudioError;

pub type AudioResult<T> = Result<T, Report<AudioError>>;

pub trait Beeper {
    fn beep(&self) -> AudioResult<()>;
}

pub struct SimpleBeeper {
    volume: f32,
    duration: Duration,
}

impl Default for SimpleBeeper {
    fn default() -> Self {
        Self {
            volume: 0.4,
            duration: Duration::from_millis(800),
        }
    }
}

impl Beeper for SimpleBeeper {
    fn beep(&self) -> AudioResult<()> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .change_context(AudioError)
            .attach("cannot open audio output")?;
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        sink.set_volume(self.volume);
        sink.append(rodio::source::SineWave::new(932.));
        thread::sleep(self.duration);
        sink.stop();
        Ok(())
    }
}
