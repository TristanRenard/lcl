use crate::pitch::PitchExpr;
use crate::span::Span;
use crate::waveform::Waveform;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    // --- Pitch ---
    SetPitch(PitchExpr),
    OctaveRel(i32),
    SemitoneRel(i32),

    // --- Volume ---
    VolumeUp(u32),
    VolumeDown(u32),
    VolumeSet(u32),

    // --- Timing ---
    Play(u32),
    Silence(u32),

    // --- Timbre ---
    SetTimbre(Waveform),

    // --- Contextes ---
    Loop { count: Option<u32>, body: Vec<Node> },
    SampleCtx(Vec<Node>),
    SynthCtx(Vec<Node>),
    MidiCtx(Vec<Node>),
    DurationCtx { duration: u32, body: Vec<Node> },
}