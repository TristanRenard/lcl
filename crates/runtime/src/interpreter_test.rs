use lcl_lexer::Lexer;
use lcl_parser::Parser;
use crate::interpreter::Interpreter;
use crate::event::AudioEvent;

fn run(input: &str) -> Vec<AudioEvent> {
    let tokens = Lexer::new(input).tokenize();
    let program = Parser::new(tokens).parse();
    Interpreter::new().run(&program)
}

#[test]
fn play_c4() {
    let events = run("c4.");
    assert_eq!(events, vec![AudioEvent::PlayNote {
        pitch_midi: 60,
        velocity: 5,
        duration: 1,
        waveform: 0,
        sample_mode: false,
    }]);
}

#[test]
fn play_midi_60() {
    let events = run("{60.}");
    assert_eq!(events, vec![AudioEvent::PlayNote {
        pitch_midi: 60,
        velocity: 5,
        duration: 1,
        waveform: 0,
        sample_mode: false,
    }]);
}

#[test]
fn volume_set() {
    let events = run("c4|8=.");
    assert_eq!(events[0].clone(), AudioEvent::PlayNote {
        pitch_midi: 60,
        velocity: 8,
        duration: 1,
        waveform: 0,
        sample_mode: false,
    });
}

#[test]
fn silence() {
    let events = run("3,.");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0], AudioEvent::Silence { duration: 3 });
}

#[test]
fn sample_context() {
    let events = run("*[c4.]");
    assert_eq!(events[0].clone(), AudioEvent::PlayNote {
        pitch_midi: 60,
        velocity: 5,
        duration: 1,
        waveform: 0,
        sample_mode: true,
    });
}

#[test]
fn loop_3_times() {
    let events = run("3[c4.]");
    assert_eq!(events.len(), 3);
}

#[test]
fn duration_context() {
    let events = run("4_[c4.]");
    assert_eq!(events[0].clone(), AudioEvent::PlayNote {
        pitch_midi: 60,
        velocity: 5,
        duration: 4,
        waveform: 0,
        sample_mode: false,
    });
}

#[test]
fn sharp_note() {
    let events = run("c#4.");
    assert_eq!(events[0].clone(), AudioEvent::PlayNote {
        pitch_midi: 61,
        velocity: 5,
        duration: 1,
        waveform: 0,
        sample_mode: false,
    });
}