// Answer 0

#[test]
fn test_nest_limit_zero() {
    let result = RegexBuilder::new(r"a").nest_limit(0).build();
}

#[test]
fn test_nest_limit_one() {
    let result = RegexBuilder::new(r"ab").nest_limit(1).build();
}

#[test]
fn test_nest_limit_two() {
    let result = RegexBuilder::new(r"(ab)").nest_limit(2).build();
}

#[test]
fn test_nest_limit_maximum() {
    let result = RegexBuilder::new(r"((a)(b))").nest_limit(u32::MAX).build();
}

