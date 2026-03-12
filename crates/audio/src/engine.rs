use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;
use lcl_runtime::AudioEvent;

pub struct AudioEngine {
    _stream: OutputStream,
    handle: OutputStreamHandle,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("no audio output");
        Self {
            _stream: stream,
            handle,
        }
    }

    pub fn play_events(&self, events: &[AudioEvent]) {
        for event in events {
            match event {
                AudioEvent::PlayNote { pitch_midi, velocity, duration, waveform, sample_mode } => {
                    if *sample_mode {
                        // samples : plus tard
                        continue;
                    }
                    let freq = midi_to_freq(*pitch_midi);
                    let dur = Duration::from_millis(*duration as u64 * 500);
                    let source = rodio::source::SineWave::new(freq);
                    let sink = Sink::try_new(&self.handle).unwrap();
                    sink.set_volume(*velocity as f32 / 10.0);
                    sink.append(source);
                    std::thread::sleep(dur);
                    sink.stop();
                }
                AudioEvent::Silence { duration } => {
                    std::thread::sleep(Duration::from_millis(*duration as u64 * 500));
                }
            }
        }
    }
}

fn midi_to_freq(midi: u8) -> f32 {
    440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0)
}