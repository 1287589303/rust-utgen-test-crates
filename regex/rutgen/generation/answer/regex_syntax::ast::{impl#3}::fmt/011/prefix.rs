// Answer 0

#[test]
fn test_fmt_nest_limit_exceeded_lower_bound() {
    let limit = 1;
    let error = regex_syntax::ErrorKind::NestLimitExceeded(limit);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_nest_limit_exceeded_middle() {
    let limit = 500;
    let error = regex_syntax::ErrorKind::NestLimitExceeded(limit);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_nest_limit_exceeded_upper_bound() {
    let limit = 1000;
    let error = regex_syntax::ErrorKind::NestLimitExceeded(limit);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

