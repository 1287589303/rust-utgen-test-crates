// Answer 0

#[test]
fn test_char_normal() {
    let input: char = 'a';
    let result = input.char();
}

#[test]
fn test_char_uppercase() {
    let input: char = 'Z';
    let result = input.char();
}

#[test]
fn test_char_digit() {
    let input: char = '5';
    let result = input.char();
}

#[test]
fn test_char_special_space() {
    let input: char = ' ';
    let result = input.char();
}

#[test]
fn test_char_special_comma() {
    let input: char = ',';
    let result = input.char();
}

#[test]
fn test_char_special_period() {
    let input: char = '.';
    let result = input.char();
}

#[test]
fn test_char_lowest_ascii() {
    let input: char = '\x00';
    let result = input.char();
}

#[test]
fn test_char_highest_ascii() {
    let input: char = '\x7F';
    let result = input.char();
}

