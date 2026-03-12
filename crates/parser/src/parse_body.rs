use lcl_lexer::Token;
use lcl_ast::{Node, NodeKind};
use crate::parser::Parser;

impl Parser {
    pub(crate) fn parse_body(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::CloseBracket | Token::CloseBrace => break,
                _ => {
                    if let Some(node) = self.parse_node() {
                        nodes.push(node);
                    }
                }
            }
        }

        nodes
    }

    pub(crate) fn parse_node(&mut self) -> Option<Node> {
        let token = self.peek()?.clone();

        match token {
            Token::Dot => {
                self.advance();
                Some(Node { kind: NodeKind::Play(1), span: self.span() })
            }
            Token::Pipe => {
                self.advance();
                None
            }
            Token::Number(_) => self.parse_number(),
            Token::Note { .. } => self.parse_note(),
            Token::Star => self.parse_sample_ctx(),
            Token::Backslash => self.parse_synth_ctx(),
            Token::OpenBracket => self.parse_loop(None),
            Token::OpenBrace => self.parse_midi_ctx(None),
            Token::Greater => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeUp(1), span: self.span() })
            }
            Token::Less => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeDown(1), span: self.span() })
            }
            Token::Equal => {
                self.advance();
                Some(Node { kind: NodeKind::VolumeSet(5), span: self.span() })
            }
            Token::Comma => {
                self.advance();
                Some(Node { kind: NodeKind::Silence(1), span: self.span() })
            }
            Token::Tilde => {
                self.advance();
                Some(Node { kind: NodeKind::SetTimbre(lcl_ast::Waveform::Sine), span: self.span() })
            }
            _ => {
                self.advance();
                None
            }
        }
    }
}