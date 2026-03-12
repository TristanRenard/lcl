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