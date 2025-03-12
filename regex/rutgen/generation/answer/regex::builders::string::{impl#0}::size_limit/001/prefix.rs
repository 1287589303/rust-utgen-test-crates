// Answer 0

#[test]
fn test_size_limit_zero() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(0);
    let _ = builder.build();
}

#[test]
fn test_size_limit_one() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(1);
    let _ = builder.build();
}

#[test]
fn test_size_limit_45000() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(45000);
    let _ = builder.build();
}

#[test]
fn test_size_limit_exceed_limit() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(45001);
    let _ = builder.build();
}

