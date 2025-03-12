// Answer 0

#[test]
fn test_fmt_empty_string() {
    let regex = Regex {
        meta: meta::Regex::new("").unwrap(),
        pattern: Arc::from(""),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = regex.fmt(&mut buffer);
}

#[test]
fn test_fmt_single_character() {
    let regex = Regex {
        meta: meta::Regex::new("a").unwrap(),
        pattern: Arc::from("a"),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = regex.fmt(&mut buffer);
}

#[test]
fn test_fmt_simple_pattern() {
    let regex = Regex {
        meta: meta::Regex::new("abc").unwrap(),
        pattern: Arc::from("abc"),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = regex.fmt(&mut buffer);
}

#[test]
fn test_fmt_complex_pattern() {
    let regex = Regex {
        meta: meta::Regex::new(r"\d{2,4}-[A-Z]{3}").unwrap(),
        pattern: Arc::from(r"\d{2,4}-[A-Z]{3}"),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = regex.fmt(&mut buffer);
}

#[test]
fn test_fmt_long_pattern() {
    let regex = Regex {
        meta: meta::Regex::new(r"^(?:[a-z]{3,})$").unwrap(),
        pattern: Arc::from(r"^(?:[a-z]{3,})$"),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = regex.fmt(&mut buffer);
}

