// Answer 0

#[test]
fn test_octal_true() {
    let mut builder = RegexSetBuilder::new(["\\141"]);
    let result = builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut builder = RegexSetBuilder::new(["\\141"]);
    let result = builder.octal(false);
}

