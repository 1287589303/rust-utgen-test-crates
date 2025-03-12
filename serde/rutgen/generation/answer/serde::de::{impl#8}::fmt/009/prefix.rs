// Answer 0

#[test]
fn test_one_name() {
    let names: &'static [&'static str] = &["value1"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_empty_names() {
    let names: &'static [&'static str] = &[];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

