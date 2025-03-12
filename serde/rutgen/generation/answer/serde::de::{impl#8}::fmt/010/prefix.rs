// Answer 0

#[test]
fn test_one_of_fmt_empty_names() {
    let one_of = OneOf {
        names: &[],
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

