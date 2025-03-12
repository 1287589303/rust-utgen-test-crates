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
fn test_nest_limit_ten() {
    let result = RegexBuilder::new(r"((a)(b)(c)(d)(e)(f)(g)(h)(i)(j))").nest_limit(10).build();
}

#[test]
fn test_nest_limit_max() {
    let result = RegexBuilder::new(r"(((((a)))))").nest_limit(u32::MAX).build();
}

