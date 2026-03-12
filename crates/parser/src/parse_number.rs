use lcl_lexer::Token;
use lcl_ast::{Node, NodeKind, Waveform};
use crate::parser::Parser;

impl Parser {
    pub(crate) fn parse_number(&mut self) -> Option<Node> {
        let Token::Number(n) = self.advance()?.clone() else { return None };
        let span = self.span();

        match self.peek() {
            Some(Token::Dot) => {
                Some(Node { kind: NodeKind::SetPitch(lcl_ast::PitchExpr::Midi(n)), span })
            }
            Some(Token::Pipe) => {
                Some(Node { kind: NodeKind::SetPitch(lcl_ast::PitchExpr::Midi(n)), span })
            }
            Some(Token::Plus) => {
                self.advance();
                Some(Node { kind: NodeKind::OctaveRel(n as i32), span })
            }
            Some(Token::Minus) => {
                self.advance();
                Some(Node { kind: NodeKind::OctaveRel(-(n as i32)), span })
            }
            Some(Token::Dollar) => {
                self.advance();
                match self.peek() {
                    Some(Token::Plus) => {
                        self.advance();
                        Some(Node { kind: NodeKind::SemitoneRel(n as i32), span })
                    }
                    Some(Token::Minus) => {
                        self.advance();
                        Some(Node { kind: NodeKind::SemitoneRel(-(n as i32)), span })
                    }
                    _ => None,
                }
            }
            Some(Token::Greater) => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeUp(n), span })
            }
            Some(Token::Less) => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeDown(n), span })
            }
            Some(Token::Equal) => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeSet(n), span })
            }
            Some(Token::Comma) => {
                self.advance();
                Some(Node { kind: NodeKind::Silence(n), span })
            }
            Some(Token::Tilde) => {
                self.advance();
                let waveform = match n {
                    0 => Waveform::Sine,
                    1 => Waveform::Saw,
                    2 => Waveform::Square,
                    3 => Waveform::Triangle,
                    _ => Waveform::Sine,
                };
                Some(Node { kind: NodeKind::SetTimbre(waveform), span })
            }
            Some(Token::Underscore) => {
                self.advance();
                self.parse_duration_ctx(n)
            }
            Some(Token::OpenBracket) => {
                self.parse_loop(Some(n))
            }
            Some(Token::OpenBrace) => {
                self.parse_midi_ctx(Some(n))
            }
            Some(Token::Star) => {
                self.advance();
                self.parse_sample_ctx_with_count(n)
            }
            Some(Token::Backslash) => {
                self.advance();
                self.parse_synth_ctx_with_count(n)
            }
            _ => {
                Some(Node { kind: NodeKind::SetPitch(lcl_ast::PitchExpr::Midi(n)), span })
            }
        }
    }
}