// Answer 0

#[test]
fn test_case_insensitive_true() {
    let mut builder = Builder::new(vec!["pattern1", "pattern2"]);
    builder.case_insensitive(true);
}

#[test]
fn test_case_insensitive_false() {
    let mut builder = Builder::new(vec!["pattern1", "pattern2"]);
    builder.case_insensitive(false);
}

