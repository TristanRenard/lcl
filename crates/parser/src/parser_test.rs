use lcl_lexer::Lexer;
use lcl_ast::*;
use crate::parser::Parser;

fn parse(input: &str) -> Program {
    let tokens = Lexer::new(input).tokenize();
    Parser::new(tokens).parse()
}

#[test]
fn single_note() {
    let program = parse("c4.");
    assert_eq!(program.body.len(), 2);
    assert_eq!(program.body[0].kind, NodeKind::SetPitch(PitchExpr::Note(NotePitch {
        letter: NoteLetter::C,
        accidental: Accidental::Natural,
        octave: Some(4),
    })));
    assert_eq!(program.body[1].kind, NodeKind::Play(1));
}

#[test]
fn midi_pitch() {
    let program = parse("60.");
    assert_eq!(program.body.len(), 2);
    assert_eq!(program.body[0].kind, NodeKind::SetPitch(PitchExpr::Midi(60)));
    assert_eq!(program.body[1].kind, NodeKind::Play(1));
}

#[test]
fn volume_set() {
    let program = parse("5=.");
    assert_eq!(program.body.len(), 2);
    assert_eq!(program.body[0].kind, NodeKind::VolumeSet(5));
    assert_eq!(program.body[1].kind, NodeKind::Play(1));
}

#[test]
fn silence() {
    let program = parse("13,.");
    assert_eq!(program.body.len(), 2);
    assert_eq!(program.body[0].kind, NodeKind::Silence(13));
    assert_eq!(program.body[1].kind, NodeKind::Play(1));
}

#[test]
fn simple_loop() {
    let program = parse("3[c4.]");
    assert_eq!(program.body.len(), 1);
    match &program.body[0].kind {
        NodeKind::Loop { count, body } => {
            assert_eq!(*count, Some(3));
            assert_eq!(body.len(), 2);
        }
        _ => panic!("expected Loop"),
    }
}

#[test]
fn infinite_loop() {
    let program = parse("[c4.]");
    assert_eq!(program.body.len(), 1);
    match &program.body[0].kind {
        NodeKind::Loop { count, body } => {
            assert_eq!(*count, None);
            assert_eq!(body.len(), 2);
        }
        _ => panic!("expected Loop"),
    }
}

#[test]
fn sample_context() {
    let program = parse("*[c4.]");
    assert_eq!(program.body.len(), 1);
    match &program.body[0].kind {
        NodeKind::SampleCtx(body) => {
            assert_eq!(body.len(), 2);
        }
        _ => panic!("expected SampleCtx"),
    }
}

#[test]
fn semitone_rel() {
    let program = parse("4$+.");
    assert_eq!(program.body.len(), 2);
    assert_eq!(program.body[0].kind, NodeKind::SemitoneRel(4));
    assert_eq!(program.body[1].kind, NodeKind::Play(1));
}