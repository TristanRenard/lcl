use crate::note::NotePitch;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchExpr {
    Note(NotePitch),
    Midi(u32),
}