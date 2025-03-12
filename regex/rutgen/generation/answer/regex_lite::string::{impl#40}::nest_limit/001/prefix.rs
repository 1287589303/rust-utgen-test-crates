// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut builder = RegexBuilder::new(r"");
    builder.nest_limit(0);
    let result = builder.build();
}

#[test]
fn test_nest_limit_one() {
    let mut builder = RegexBuilder::new(r"a");
    builder.nest_limit(1);
    let result = builder.build();
}

#[test]
fn test_nest_limit_two() {
    let mut builder = RegexBuilder::new(r"(a)");
    builder.nest_limit(2);
    let result = builder.build();
}

#[test]
fn test_nest_limit_three() {
    let mut builder = RegexBuilder::new(r"((a))");
    builder.nest_limit(3);
    let result = builder.build();
}

#[test]
fn test_nest_limit_max() {
    let mut builder = RegexBuilder::new(r"(((a)))");
    builder.nest_limit(u32::MAX);
    let result = builder.build();
}

