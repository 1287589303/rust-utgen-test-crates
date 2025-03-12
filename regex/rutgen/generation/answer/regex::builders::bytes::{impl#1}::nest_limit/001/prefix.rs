// Answer 0

#[test]
fn test_nest_limit_zero() {
    let patterns = vec![r"a"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(0).build().unwrap();
}

#[test]
fn test_nest_limit_one() {
    let patterns = vec![r"ab"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(1).build().unwrap();
}

#[test]
fn test_nest_limit_boundary_max() {
    let patterns = vec![r"x"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(u32::MAX).build().unwrap();
}

#[test]
#[should_panic]
fn test_nest_limit_exceed() {
    let patterns = vec![r"abc"];
    let builder = RegexSetBuilder::new(patterns);
    builder.nest_limit(0).build().unwrap();
}

