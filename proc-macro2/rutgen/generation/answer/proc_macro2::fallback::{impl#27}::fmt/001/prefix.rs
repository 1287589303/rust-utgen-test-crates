// Answer 0

#[test]
fn test_fmt_with_empty_repr() {
    let literal = Literal { repr: String::from(""), span: Span { lo: 0, hi: 0 } };
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    literal.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_valid_repr() {
    let literal = Literal { repr: String::from("valid_string"), span: Span { lo: 1, hi: 5 } };
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    literal.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_span_locations_enabled() {
    #[cfg(span_locations)]
    {
        let literal = Literal { repr: String::from("string_with_span"), span: Span { lo: 2, hi: 6 } };
        let mut buffer = String::new();
        let mut formatter = fmt::Formatter::new(&mut buffer);
        literal.fmt(&mut formatter).unwrap();
    }
}

#[test]
fn test_fmt_with_span_locations_disabled() {
    #[cfg(not(span_locations))]
    {
        let literal = Literal { repr: String::from("another_string"), span: Span { lo: 3, hi: 7 } };
        let mut buffer = String::new();
        let mut formatter = fmt::Formatter::new(&mut buffer);
        literal.fmt(&mut formatter).unwrap();
    }
}

