// Answer 0

#[test]
fn test_nest_limit_zero() {
    let patterns = vec![r"a"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(0).build();
}

#[test]
fn test_nest_limit_one() {
    let patterns = vec![r"ab"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(1).build();
}

#[test]
fn test_nest_limit_ten() {
    let patterns = vec![r"abcde"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(10).build();
}

#[test]
fn test_nest_limit_one_hundred() {
    let patterns = vec![r"abcdefghij"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(100).build();
}

#[test]
fn test_nest_limit_one_thousand() {
    let patterns = vec![r"a".repeat(1000)];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(1000).build();
}

#[test]
fn test_nest_limit_max_u32() {
    let patterns = vec![r"a"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(u32::MAX).build();
}

