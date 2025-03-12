// Answer 0

#[test]
fn test_repeat_char_with_zero_count() {
    let c = 'a';
    let count = 0;
    repeat_char(c, count);
}

#[test]
fn test_repeat_char_with_one_count() {
    let c = 'Z';
    let count = 1;
    repeat_char(c, count);
}

#[test]
fn test_repeat_char_with_large_count() {
    let c = '1';
    let count = 1000;
    repeat_char(c, count);
}

#[test]
fn test_repeat_char_with_special_char() {
    let c = '!';
    let count = 5;
    repeat_char(c, count);
}

#[test]
fn test_repeat_char_with_boundary_case() {
    let c = ' ';
    let count = 2;
    repeat_char(c, count);
}

#[test]
fn test_repeat_char_with_non_ascii_char() {
    let c = 'é';
    let count = 3;
    repeat_char(c, count);
}

