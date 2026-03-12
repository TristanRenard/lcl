#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Litéraux
    Number(u32),
    Note { letter: char, accidental: Option<char>, octave: Option<u8> },

    // Opérateurs
    Dot,        // .
    Plus,       // +
    Minus,      // -
    Dollar,     // $
    Greater,    // >
    Less,       // 
    Equal,      // =
    Comma,      // ,
    Tilde,      // ~
    Pipe,       // |
    Underscore, // _

    // Délimiteurs
    OpenBracket,    // [
    CloseBracket,   // ]
    OpenBrace,      // {
    CloseBrace,     // }

    // Contextes
    Star,       // *
    Backslash,  // \
}