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
fn test_size_limit_fifty() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(50);
    let _ = builder.build();
}

#[test]
fn test_size_limit_hundred() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(100);
    let _ = builder.build();
}

#[test]
fn test_size_limit_thousand() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(1000);
    let _ = builder.build();
}

#[test]
fn test_size_limit_ten_thousand() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(10000);
    let _ = builder.build();
}

#[test]
fn test_size_limit_max_usize() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.size_limit(std::usize::MAX);
    let _ = builder.build();
}

