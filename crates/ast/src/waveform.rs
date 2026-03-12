#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,     // 0~
    Saw,      // 1~
    Square,   // 2~
    Triangle, // 3~
}