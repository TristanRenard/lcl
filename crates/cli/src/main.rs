use lcl_lexer::Lexer;
use lcl_parser::Parser;
use lcl_runtime::Interpreter;
use lcl_audio::AudioEngine;

fn main() {
    let input = "c4. 2_[f4.] 3_[c4.] 3_[f4.] 3_[g4.]";

    let tokens = Lexer::new(input).tokenize();
    let program = Parser::new(tokens).parse();
    let events = Interpreter::new().run(&program);

    let engine = AudioEngine::new();
    engine.play_events(&events);
}