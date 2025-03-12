// Answer 0

#[test]
fn test_string_empty_replacement() {
    let mut dst = String::new();
    string(
        "",
        |_, _| {
            // Should not be called
        },
        |_| {
            // Should not be called
            None
        },
        &mut dst,
    );
}

#[test]
fn test_string_replacement_with_only_escape() {
    let mut dst = String::new();
    string(
        "$$",
        |_, _| {
            // Should not be called
        },
        |_| {
            // Should not be called
            None
        },
        &mut dst,
    );
}

#[test]
fn test_string_replacement_with_no_capture() {
    let mut dst = String::new();
    string(
        "test string",
        |_, _| {
            // Should not be called
        },
        |_| {
            // Should not be called
            None
        },
        &mut dst,
    );
}

