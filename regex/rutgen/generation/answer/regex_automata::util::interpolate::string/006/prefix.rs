// Answer 0

#[test]
fn test_string_single_named_capture() {
    let mut dst = String::new();
    string(
        "foo $bar baz",
        |index, dst| {
            if index == 0 {
                dst.push_str("BAR");
            }
        },
        |name| {
            if name == "bar" {
                Some(0)
            } else {
                None
            }
        },
        &mut dst,
    );
}

#[test]
fn test_string_multiple_replacements() {
    let mut dst = String::new();
    string(
        "Hello $name, welcome to $place.",
        |index, dst| {
            if index == 0 {
                dst.push_str("Alice");
            } else if index == 1 {
                dst.push_str("Wonderland");
            }
        },
        |name| {
            if name == "name" {
                Some(0)
            } else if name == "place" {
                Some(1)
            } else {
                None
            }
        },
        &mut dst,
    );
}

#[test]
fn test_string_ignore_nonexistent_name() {
    let mut dst = String::new();
    string(
        "This is a test for $nonexistent and $valid.",
        |index, dst| {
            if index == 1 {
                dst.push_str("VALID");
            }
        },
        |name| {
            if name == "valid" {
                Some(1)
            } else {
                None
            }
        },
        &mut dst,
    );
}

