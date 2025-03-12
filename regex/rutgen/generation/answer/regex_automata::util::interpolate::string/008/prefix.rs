// Answer 0

#[test]
fn test_string_with_single_capture_number() {
    let mut dst = String::new();
    crate::util::interpolate::string(
        "$0",
        |index, dst| {
            if index == 0 {
                dst.push_str("VALUE0");
            }
        },
        |name| None,
        &mut dst,
    );
}

#[test]
fn test_string_with_single_capture_name() {
    let mut dst = String::new();
    crate::util::interpolate::string(
        "$name",
        |index, dst| {
            if index == 0 {
                dst.push_str("NAMED_VALUE");
            }
        },
        |name| {
            if name == "name" {
                Some(0)
            } else {
                None
            }
        },
        &mut dst,
    );
}

#[test]
fn test_string_with_multiple_references() {
    let mut dst = String::new();
    crate::util::interpolate::string(
        "Start $0 middle $name end",
        |index, dst| {
            if index == 0 {
                dst.push_str("FIRST");
            }
        },
        |name| {
            if name == "name" {
                Some(0)
            } else {
                None
            }
        },
        &mut dst,
    );
}

#[test]
fn test_string_with_no_valid_capture() {
    let mut dst = String::new();
    crate::util::interpolate::string(
        "Hello $xyz",
        |index, dst| {},
        |name| None,
        &mut dst,
    );
}

#[test]
fn test_string_with_escape_sequence() {
    let mut dst = String::new();
    crate::util::interpolate::string(
        "Value $$ is a thing",
        |index, dst| {},
        |name| None,
        &mut dst,
    );
}

