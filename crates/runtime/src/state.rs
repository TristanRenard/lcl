pub struct State {
    pub pitch_midi: u8,
    pub volume: u8,
    pub duration: u32,
    pub waveform: u8,
    pub sample_mode: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            pitch_midi: 60, // c4
            volume: 5,
            duration: 1,
            waveform: 0, // sine
            sample_mode: false,
        }
    }
}