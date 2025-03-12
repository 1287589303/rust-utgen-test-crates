// Answer 0

#[test]
fn test_decrement_at_e000() {
    let input: char = '\u{E000}';
    let result = input.decrement();
}

#[test]
fn test_decrement_at_d7ff() {
    let input: char = '\u{D7FF}';
    let result = input.decrement();
}

