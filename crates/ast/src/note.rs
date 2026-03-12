

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteLetter {
    C, D, E, F, G, A, B,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    Natural,
    Sharp, // #
    Flat,  // `
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotePitch {
    pub letter: NoteLetter,
    pub accidental: Accidental,
    pub octave: Option<u8>,
}