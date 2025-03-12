// Answer 0

#[test]
fn test_string_with_single_capture_number() {
    let mut dst = String::new();
    super::string(
        "$0",
        |index, dst| {
            if index == 0 {
                dst.push_str("CaptureZero");
            }
        },
        |name| None,
        &mut dst,
    );
}

#[test]
fn test_string_with_multiple_captures() {
    let mut dst = String::new();
    super::string(
        "Start $0 middle $1 end",
        |index, dst| {
            match index {
                0 => dst.push_str("FirstCapture"),
                1 => dst.push_str("SecondCapture"),
                _ => {}
            }
        },
        |name| {
            if name == "first" {
                Some(0)
            } else if name == "second" {
                Some(1)
            } else {
                None
            }
        },
        &mut dst,
    );
}

#[test]
fn test_string_with_escaped_dollar() {
    let mut dst = String::new();
    super::string(
        "Value: $$0 is not the same as $1",
        |index, dst| {
            if index == 1 {
                dst.push_str("CaptureOne");
            }
        },
        |name| None,
        &mut dst,
    );
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    super::string(
        "Hello $name",
        |index, dst| {
            if index == 0 {
                dst.push_str("World");
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
fn test_string_with_no_valid_captures() {
    let mut dst = String::new();
    super::string(
        "Just testing $invalid_name",
        |index, dst| {},
        |name| None,
        &mut dst,
    );
}

#[test]
fn test_string_with_empty_replacement() {
    let mut dst = String::new();
    super::string(
        "",
        |index, dst| {},
        |name| None,
        &mut dst,
    );
}

