// Answer 0

#[test]
fn test_fmt_with_empty_exact_literal() {
    let lit = Literal::exact(Vec::new());
    let _ = core::fmt::format(format_args!("{:?}", lit));
}

#[test]
fn test_fmt_with_single_element_exact_literal() {
    let lit = Literal::exact(Vec::from([0x01]));
    let _ = core::fmt::format(format_args!("{:?}", lit));
}

#[test]
fn test_fmt_with_multiple_elements_exact_literal() {
    let lit = Literal::exact(Vec::from([0x01, 0x02, 0x03]));
    let _ = core::fmt::format(format_args!("{:?}", lit));
}

