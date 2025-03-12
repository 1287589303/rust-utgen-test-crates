// Answer 0

#[test]
fn test_size_limit_zero() {
    let mut builder = RegexSetBuilder::new(["pattern1", "pattern2"]);
    builder.size_limit(0);
}

#[test]
fn test_size_limit_one() {
    let mut builder = RegexSetBuilder::new(["pattern1", "pattern2"]);
    builder.size_limit(1);
}

#[test]
fn test_size_limit_max() {
    let mut builder = RegexSetBuilder::new(["pattern1", "pattern2"]);
    builder.size_limit(u32::MAX as usize);
}

#[test]
fn test_size_limit_exceed() {
    let mut builder = RegexSetBuilder::new(["pattern1", "pattern2"]);
    builder.size_limit(u32::MAX as usize + 1);
}

#[test]
fn test_size_limit_mid_value() {
    let mut builder = RegexSetBuilder::new(["pattern1", "pattern2"]);
    builder.size_limit(10_000);
}

