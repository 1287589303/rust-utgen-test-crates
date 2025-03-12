// Answer 0

#[test]
fn test_replace_append_empty_string() {
    let mut s = String::new();
    let mut dst = String::new();
    let caps = Captures {
        haystack: "",
        slots: CaptureLocations::default(),
        pikevm: Arc::new(PikeVM::default()),
    };
    s.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_special_characters() {
    let mut s = String::from("!@#$%^&*()");
    let mut dst = String::new();
    let caps = Captures {
        haystack: "!@#$%",
        slots: CaptureLocations::default(),
        pikevm: Arc::new(PikeVM::default()),
    };
    s.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_whitespace_string() {
    let mut s = String::from("   ");
    let mut dst = String::new();
    let caps = Captures {
        haystack: "  ",
        slots: CaptureLocations::default(),
        pikevm: Arc::new(PikeVM::default()),
    };
    s.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_normal_sentence() {
    let mut s = String::from("Hello world");
    let mut dst = String::new();
    let caps = Captures {
        haystack: "Hello",
        slots: CaptureLocations::default(),
        pikevm: Arc::new(PikeVM::default()),
    };
    s.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_non_matching_pattern() {
    let mut s = String::from("Goodbye");
    let mut dst = String::new();
    let caps = Captures {
        haystack: "Hello",
        slots: CaptureLocations::default(),
        pikevm: Arc::new(PikeVM::default()),
    };
    s.replace_append(&caps, &mut dst);
}

