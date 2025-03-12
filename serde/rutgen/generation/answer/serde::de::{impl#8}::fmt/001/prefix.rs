// Answer 0

#[test]
#[should_panic]
fn test_fmt_names_len_zero() {
    let names: &'static [&'static str] = &[];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    one_of.fmt(&mut formatter);
}

#[test]
fn test_fmt_names_len_one() {
    let names: &'static [&'static str] = &["name1"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

#[test]
fn test_fmt_names_len_two() {
    let names: &'static [&'static str] = &["name1", "name2"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

#[test]
fn test_fmt_names_len_more_than_two() {
    let names: &'static [&'static str] = &["name1", "name2", "name3"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    let _ = one_of.fmt(&mut formatter);
}

