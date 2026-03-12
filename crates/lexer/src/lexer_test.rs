use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn single_dot() {
    let tokens = Lexer::new(".").tokenize();
    assert_eq!(tokens, vec![Token::Dot]);
}

#[test]
fn number() {
    let tokens = Lexer::new("123").tokenize();
    assert_eq!(tokens, vec![Token::Number(123)]);
}

#[test]
fn note_simple() {
    let tokens = Lexer::new("c").tokenize();
    assert_eq!(tokens, vec![Token::Note { letter: 'c', accidental: None, octave: None }]);
}

#[test]
fn note_sharp_octave() {
    let tokens = Lexer::new("c#4").tokenize();
    assert_eq!(tokens, vec![Token::Note { letter: 'c', accidental: Some('#'), octave: Some(4) }]);
}

#[test]
fn note_flat() {
    let tokens = Lexer::new("d`3").tokenize();
    assert_eq!(tokens, vec![Token::Note { letter: 'd', accidental: Some('`'), octave: Some(3) }]);
}

#[test]
fn full_expression() {
    let tokens = Lexer::new("c4.2+.").tokenize();
    assert_eq!(tokens, vec![
        Token::Note { letter: 'c', accidental: None, octave: Some(4) },
        Token::Dot,
        Token::Number(2),
        Token::Plus,
        Token::Dot,
    ]);
}

#[test]
fn everything() {
    let tokens = Lexer::new("3[c#4. 2+. d`3 5> 1< 2= 4, 1~ | _ * \\ { 60 } $-]").tokenize();
    assert_eq!(tokens, vec![
        Token::Number(3),
        Token::OpenBracket,
        Token::Note { letter: 'c', accidental: Some('#'), octave: Some(4) },
        Token::Dot,
        Token::Number(2),
        Token::Plus,
        Token::Dot,
        Token::Note { letter: 'd', accidental: Some('`'), octave: Some(3) },
        Token::Number(5),
        Token::Greater,
        Token::Number(1),
        Token::Less,
        Token::Number(2),
        Token::Equal,
        Token::Number(4),
        Token::Comma,
        Token::Number(1),
        Token::Tilde,
        Token::Pipe,
        Token::Underscore,
        Token::Star,
        Token::Backslash,
        Token::OpenBrace,
        Token::Number(60),
        Token::CloseBrace,
        Token::Dollar,
        Token::Minus,
        Token::CloseBracket,
    ]);
}