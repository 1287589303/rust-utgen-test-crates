// Answer 0

#[test]
#[should_panic]
fn test_one_of_empty_names() {
    let names: &'static [&'static str] = &[];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    one_of.fmt(&mut formatter);
}

#[test]
fn test_one_of_single_name() {
    let names: &'static [&'static str] = &["name1"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    one_of.fmt(&mut formatter);
}

#[test]
fn test_one_of_two_names() {
    let names: &'static [&'static str] = &["name1", "name2"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    one_of.fmt(&mut formatter);
}

#[test]
fn test_one_of_multiple_names() {
    let names: &'static [&'static str] = &["name1", "name2", "name3"];
    let one_of = OneOf { names };
    let mut formatter = std::fmt::Formatter::new();
    one_of.fmt(&mut formatter);
}

