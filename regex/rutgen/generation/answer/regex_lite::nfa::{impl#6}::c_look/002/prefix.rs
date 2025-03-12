// Answer 0

#[test]
fn test_c_look_start() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::Start;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_end() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::End;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_startlf() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::StartLF;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_endlf() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::EndLF;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_startcrlf() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::StartCRLF;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_endcrlf() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::EndCRLF;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::Word;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word_negate() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::WordNegate;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word_start() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::WordStart;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word_end() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::WordEnd;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word_start_half() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::WordStartHalf;
    let _result = compiler.c_look(&look);
}

#[test]
fn test_c_look_word_end_half() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let look = Look::WordEndHalf;
    let _result = compiler.c_look(&look);
}

