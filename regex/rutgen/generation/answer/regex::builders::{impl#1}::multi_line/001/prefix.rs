// Answer 0

#[test]
fn test_multi_line_true() {
    let mut builder = Builder::new(vec!["pattern1", "pattern2"]);
    let result = builder.multi_line(true);
}

#[test]
fn test_multi_line_false() {
    let mut builder = Builder::new(vec!["pattern1", "pattern2"]);
    let result = builder.multi_line(false);
}

