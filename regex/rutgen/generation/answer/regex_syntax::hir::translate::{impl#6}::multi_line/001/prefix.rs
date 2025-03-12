// Answer 0

#[test]
fn test_multi_line_true() {
    let flags = Flags {
        multi_line: Some(true),
        ..Flags::default()
    };
    let result = flags.multi_line();
}

#[test]
fn test_multi_line_false() {
    let flags = Flags {
        multi_line: Some(false),
        ..Flags::default()
    };
    let result = flags.multi_line();
}

#[test]
fn test_multi_line_none() {
    let flags = Flags {
        multi_line: None,
        ..Flags::default()
    };
    let result = flags.multi_line();
}

