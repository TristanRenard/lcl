use lcl_lexer::Token;
use lcl_ast::{ Program, Span};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub(crate) fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    pub(crate) fn span(&self) -> Span {
        Span { offset: self.pos, line: 0, col: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let body = self.parse_body();
        Program { body }
    }
}