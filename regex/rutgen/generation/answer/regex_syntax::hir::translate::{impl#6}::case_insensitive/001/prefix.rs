// Answer 0

#[test]
fn test_case_insensitive_true() {
    let flags = Flags {
        case_insensitive: Some(true),
        ..Flags::default()
    };
    flags.case_insensitive();
}

#[test]
fn test_case_insensitive_false() {
    let flags = Flags {
        case_insensitive: Some(false),
        ..Flags::default()
    };
    flags.case_insensitive();
}

#[test]
fn test_case_insensitive_none() {
    let flags = Flags::default();
    flags.case_insensitive();
}

