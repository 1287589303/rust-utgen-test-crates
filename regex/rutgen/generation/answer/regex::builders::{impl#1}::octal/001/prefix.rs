// Answer 0

#[test]
fn test_octal_true() {
    let mut builder = Builder::new(vec!["pattern1"]).case_insensitive(true);
    builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut builder = Builder::new(vec!["pattern2"]).multi_line(true);
    builder.octal(false);
}

