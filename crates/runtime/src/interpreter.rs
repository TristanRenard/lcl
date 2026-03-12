use lcl_ast::*;
use crate::event::AudioEvent;
use crate::state::State;

pub struct Interpreter {
    state: State,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { state: State::new() }
    }

    pub fn run(&mut self, program: &Program) -> Vec<AudioEvent> {
        let mut events = Vec::new();
        self.eval_nodes(&program.body, &mut events);
        events
    }

    fn eval_nodes(&mut self, nodes: &[Node], events: &mut Vec<AudioEvent>) {
        for node in nodes {
            self.eval_node(node, events);
        }
    }

    fn eval_node(&mut self, node: &Node, events: &mut Vec<AudioEvent>) {
      match &node.kind {
          NodeKind::SetPitch(expr) => {
              self.state.pitch_midi = self.resolve_pitch(expr);
          }
          NodeKind::OctaveRel(n) => {
              self.state.pitch_midi = (self.state.pitch_midi as i32 + n * 12) as u8;
          }
          NodeKind::SemitoneRel(n) => {
              self.state.pitch_midi = (self.state.pitch_midi as i32 + n) as u8;
          }
          NodeKind::VolumeUp(n) => {
              self.state.volume = self.state.volume.saturating_add(*n as u8);
          }
          NodeKind::VolumeDown(n) => {
              self.state.volume = self.state.volume.saturating_sub(*n as u8);
          }
          NodeKind::VolumeSet(n) => {
              self.state.volume = *n as u8;
          }
          NodeKind::Play(count) => {
              for _ in 0..*count {
                  events.push(AudioEvent::PlayNote {
                      pitch_midi: self.state.pitch_midi,
                      velocity: self.state.volume,
                      duration: self.state.duration,
                      waveform: self.state.waveform,
                      sample_mode: self.state.sample_mode,
                  });
              }
          }
          NodeKind::Silence(n) => {
              events.push(AudioEvent::Silence { duration: *n });
          }
          NodeKind::SetTimbre(waveform) => {
              self.state.waveform = match waveform {
                  Waveform::Sine => 0,
                  Waveform::Saw => 1,
                  Waveform::Square => 2,
                  Waveform::Triangle => 3,
              };
          }
          NodeKind::Loop { count, body } => {
              match count {
                  Some(n) => {
                      for _ in 0..*n {
                          self.eval_nodes(body, events);
                      }
                  }
                  None => {
                      self.eval_nodes(body, events);
                  }
              }
          }
          NodeKind::SampleCtx(body) => {
              let prev = self.state.sample_mode;
              self.state.sample_mode = true;
              self.eval_nodes(body, events);
              self.state.sample_mode = prev;
          }
          NodeKind::SynthCtx(body) => {
              let prev = self.state.sample_mode;
              self.state.sample_mode = false;
              self.eval_nodes(body, events);
              self.state.sample_mode = prev;
          }
          NodeKind::MidiCtx(body) => {
              self.eval_nodes(body, events);
          }
          NodeKind::DurationCtx { duration, body } => {
              let prev = self.state.duration;
              self.state.duration = *duration;
              self.eval_nodes(body, events);
              self.state.duration = prev;
          }
      }
  }

  fn resolve_pitch(&self, expr: &PitchExpr) -> u8 {
    match expr {
        PitchExpr::Midi(n) => *n as u8,
        PitchExpr::Note(note) => {
            let base = match note.letter {
                NoteLetter::C => 0,
                NoteLetter::D => 2,
                NoteLetter::E => 4,
                NoteLetter::F => 5,
                NoteLetter::G => 7,
                NoteLetter::A => 9,
                NoteLetter::B => 11,
            };
            let accidental = match note.accidental {
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
                Accidental::Natural => 0,
            };
            let octave = note.octave.unwrap_or(4) as i32;
            ((octave + 1) * 12 + base + accidental) as u8
        }
    }
  }
}