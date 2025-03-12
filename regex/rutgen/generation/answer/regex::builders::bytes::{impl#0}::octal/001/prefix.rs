// Answer 0

#[test]
fn test_octal_true() {
    let re = RegexBuilder::new(r"\141")
        .octal(true)
        .build()
        .unwrap();
}

#[test]
fn test_octal_false() {
    let re = RegexBuilder::new(r"\141")
        .octal(false)
        .build()
        .unwrap();
}

#[test]
fn test_octal_with_empty_pattern() {
    let re = RegexBuilder::new("")
        .octal(true)
        .build()
        .unwrap();
}

#[test]
fn test_octal_with_pattern_that_cannot_have_backreference() {
    let re = RegexBuilder::new(r"\1")
        .octal(true)
        .build()
        .unwrap();
}

#[test]
fn test_octal_with_non_matching_octals() {
    let re = RegexBuilder::new(r"\151")
        .octal(true)
        .build()
        .unwrap();
    let match_result = re.is_match(b"i");
}

#[test]
fn test_octal_boundary_case() {
    let re = RegexBuilder::new(r"\377")
        .octal(true)
        .build()
        .unwrap();
    let match_result = re.is_match(b"\xff");
}

