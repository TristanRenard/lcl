#[derive(Debug, Clone, PartialEq)]
pub enum AudioEvent {
    PlayNote {
        pitch_midi: u8,
        velocity: u8,
        duration: u32,
        waveform: u8,
        sample_mode: bool,
    },
    Silence {
        duration: u32,
    },
}