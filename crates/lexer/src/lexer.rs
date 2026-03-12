use std::collections::HashMap;
use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    char_tokens: HashMap<char, Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let char_tokens: HashMap<char, Token> = [
            ('.', Token::Dot),
            ('+', Token::Plus),
            ('-', Token::Minus),
            ('$', Token::Dollar),
            ('>', Token::Greater),
            ('<', Token::Less),
            ('=', Token::Equal),
            (',', Token::Comma),
            ('~', Token::Tilde),
            ('|', Token::Pipe),
            ('_', Token::Underscore),
            ('[', Token::OpenBracket),
            (']', Token::CloseBracket),
            ('{', Token::OpenBrace),
            ('}', Token::CloseBrace),
            ('*', Token::Star),
            ('\\', Token::Backslash),
        ].into_iter().collect();

        Self {
            input: input.chars().collect(),
            pos: 0,
            char_tokens,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn read_number(&mut self) -> u32 {
        let mut n = 0u32;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                n = n * 10 + (ch as u32 - '0' as u32);
                self.advance();
            } else {
                break;
            }
        }
        n
    }

    fn read_note(&mut self) -> Token {
        let letter = self.advance().unwrap();

        let accidental = match self.peek() {
            Some('#') => { self.advance(); Some('#') }
            Some('`') => { self.advance(); Some('`') }
            _ => None,
        };

        let octave = match self.peek() {
            Some(ch) if ch.is_ascii_digit() => {
                self.advance();
                Some(ch as u8 - b'0')
            }
            _ => None,
        };

        Token::Note { letter, accidental, octave }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => { self.advance(); }
                'a'..='g' => { tokens.push(self.read_note()); }
                ch if ch.is_ascii_digit() => { tokens.push(Token::Number(self.read_number())); }
                ch => {
                  let token = self.char_tokens.get(&ch).cloned();
                  if let Some(token) = token {
                      self.advance();
                      tokens.push(token);
                  } else {
                      self.advance();
                  }
              }
            }
        }

        tokens
    }
}