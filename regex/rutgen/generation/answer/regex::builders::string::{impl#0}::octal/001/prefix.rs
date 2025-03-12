// Answer 0

#[test]
fn test_octal_mode_enabled() {
    let pattern = r"\141"; // Octal for 'a'
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(true).build().unwrap();
}

#[test]
fn test_octal_mode_disabled() {
    let pattern = r"\141"; // Octal for 'a'
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(false).build().unwrap();
}

#[test]
fn test_octal_mode_with_simple_pattern() {
    let pattern = r"\123"; // Octal for 'S'
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(true).build().unwrap();
}

#[test]
fn test_octal_mode_with_empty_pattern() {
    let pattern = ""; // Empty pattern
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(true).build().unwrap();
}

#[test]
fn test_octal_mode_with_special_characters() {
    let pattern = r"\045"; // Octal for '%'
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(true).build().unwrap();
}

#[test]
fn test_octal_mode_with_invalid_octals() {
    let pattern = r"\256"; // Invalid octal (out of range)
    let mut builder = RegexBuilder::new(pattern);
    let _ = builder.octal(true).build(); // Expect this to return an error
}

