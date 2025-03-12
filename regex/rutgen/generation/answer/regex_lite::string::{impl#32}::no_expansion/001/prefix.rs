// Answer 0

#[test]
fn test_no_expansion_empty_string() {
    let mut empty_string: Cow<str> = Cow::Borrowed("");
    empty_string.no_expansion();
}

#[test]
fn test_no_expansion_string_without_dollar() {
    let mut normal_string: Cow<str> = Cow::Borrowed("hello world");
    normal_string.no_expansion();
}

#[test]
fn test_no_expansion_string_with_dollar() {
    let mut dollar_string: Cow<str> = Cow::Borrowed("price: $100");
    dollar_string.no_expansion();
}

#[test]
fn test_no_expansion_long_string_without_dollar() {
    let mut long_string: Cow<str> = Cow::Borrowed("a".repeat(100));
    long_string.no_expansion();
}

#[test]
fn test_no_expansion_long_string_with_dollar() {
    let mut long_dollar_string: Cow<str> = Cow::Borrowed("value: $very_long_string");
    long_dollar_string.no_expansion();
}

