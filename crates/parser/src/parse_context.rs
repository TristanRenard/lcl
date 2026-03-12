use lcl_lexer::Token;
use lcl_ast::{Node, NodeKind, PitchExpr, NotePitch, NoteLetter, Accidental};
use crate::parser::Parser;

impl Parser {
    pub(crate) fn parse_loop(&mut self, count: Option<u32>) -> Option<Node> {
        self.advance(); // consomme [
        let span = self.span();
        let body = self.parse_body();
        if matches!(self.peek(), Some(Token::CloseBracket)) {
            self.advance(); // consomme ]
        }
        Some(Node { kind: NodeKind::Loop { count, body }, span })
    }

    pub(crate) fn parse_midi_ctx(&mut self, _count: Option<u32>) -> Option<Node> {
        self.advance(); // consomme {
        let span = self.span();
        let body = self.parse_body();
        if matches!(self.peek(), Some(Token::CloseBrace)) {
            self.advance(); // consomme }
        }
        Some(Node { kind: NodeKind::MidiCtx(body), span })
    }

    pub(crate) fn parse_sample_ctx(&mut self) -> Option<Node> {
        self.advance(); // consomme *
        let span = self.span();
        match self.peek() {
            Some(Token::OpenBracket) => {
                self.advance(); // consomme [
                let body = self.parse_body();
                if matches!(self.peek(), Some(Token::CloseBracket)) {
                    self.advance();
                }
                Some(Node { kind: NodeKind::SampleCtx(body), span })
            }
            Some(Token::Dot) => {
                self.advance(); // consomme .
                Some(Node { kind: NodeKind::SampleCtx(vec![
                    Node { kind: NodeKind::Play(1), span }
                ]), span })
            }
            _ => None,
        }
    }

    pub(crate) fn parse_sample_ctx_with_count(&mut self, count: u32) -> Option<Node> {
        let span = self.span();
        match self.peek() {
            Some(Token::OpenBracket) => {
                self.advance();
                let body = self.parse_body();
                if matches!(self.peek(), Some(Token::CloseBracket)) {
                    self.advance();
                }
                Some(Node { kind: NodeKind::SampleCtx(vec![
                    Node { kind: NodeKind::Loop { count: Some(count), body }, span }
                ]), span })
            }
            Some(Token::Dot) => {
                self.advance();
                Some(Node { kind: NodeKind::SampleCtx(vec![
                    Node { kind: NodeKind::Play(count), span }
                ]), span })
            }
            _ => None,
        }
    }

    pub(crate) fn parse_synth_ctx(&mut self) -> Option<Node> {
        self.advance(); // consomme \
        let span = self.span();
        match self.peek() {
            Some(Token::OpenBracket) => {
                self.advance();
                let body = self.parse_body();
                if matches!(self.peek(), Some(Token::CloseBracket)) {
                    self.advance();
                }
                Some(Node { kind: NodeKind::SynthCtx(body), span })
            }
            Some(Token::Dot) => {
                self.advance();
                Some(Node { kind: NodeKind::SynthCtx(vec![
                    Node { kind: NodeKind::Play(1), span }
                ]), span })
            }
            _ => None,
        }
    }

    pub(crate) fn parse_synth_ctx_with_count(&mut self, count: u32) -> Option<Node> {
        let span = self.span();
        match self.peek() {
            Some(Token::OpenBracket) => {
                self.advance();
                let body = self.parse_body();
                if matches!(self.peek(), Some(Token::CloseBracket)) {
                    self.advance();
                }
                Some(Node { kind: NodeKind::SynthCtx(vec![
                    Node { kind: NodeKind::Loop { count: Some(count), body }, span }
                ]), span })
            }
            Some(Token::Dot) => {
                self.advance();
                Some(Node { kind: NodeKind::SynthCtx(vec![
                    Node { kind: NodeKind::Play(count), span }
                ]), span })
            }
            _ => None,
        }
    }

    pub(crate) fn parse_duration_ctx(&mut self, duration: u32) -> Option<Node> {
        let span = self.span();
        match self.peek() {
            Some(Token::OpenBracket) => {
                self.advance();
                let body = self.parse_body();
                if matches!(self.peek(), Some(Token::CloseBracket)) {
                    self.advance();
                }
                Some(Node { kind: NodeKind::DurationCtx { duration, body }, span })
            }
            _ => None,
        }
    }

    pub(crate) fn parse_note(&mut self) -> Option<Node> {
        let token = self.advance()?.clone();
        let span = self.span();

        let Token::Note { letter, accidental, octave } = token else { return None };

        let note_letter = match letter {
            'c' => NoteLetter::C,
            'd' => NoteLetter::D,
            'e' => NoteLetter::E,
            'f' => NoteLetter::F,
            'g' => NoteLetter::G,
            'a' => NoteLetter::A,
            'b' => NoteLetter::B,
            _ => return None,
        };

        let acc = match accidental {
            Some('#') => Accidental::Sharp,
            Some('`') => Accidental::Flat,
            _ => Accidental::Natural,
        };

        let pitch = PitchExpr::Note(NotePitch {
            letter: note_letter,
            accidental: acc,
            octave,
        });

        Some(Node { kind: NodeKind::SetPitch(pitch), span })
    }
}