// Answer 0

#[test]
fn test_c0_control() {
    let input = '\u{0000}';
    c0_control_or_space(input);
}

#[test]
fn test_c0_control_boundaries() {
    let input = '\u{001F}';
    c0_control_or_space(input);
}

#[test]
fn test_c0_space() {
    let input = ' ';
    c0_control_or_space(input);
}

#[test]
fn test_c0_non_control() {
    let input = 'A';
    c0_control_or_space(input);
}

#[test]
fn test_c0_non_control_higher() {
    let input = '\u{007F}';
    c0_control_or_space(input);
}

#[test]
fn test_c0_non_control_highest() {
    let input = '\u{0021}';
    c0_control_or_space(input);
}

