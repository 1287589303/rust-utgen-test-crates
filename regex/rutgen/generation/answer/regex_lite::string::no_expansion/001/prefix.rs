// Answer 0

#[test]
fn test_no_expansion_with_single_dollar_sign() {
    let input = "$";
    no_expansion(&input);
}

#[test]
fn test_no_expansion_with_dollar_sign_at_end() {
    let input = "abc$";
    no_expansion(&input);
}

#[test]
fn test_no_expansion_with_dollar_sign_at_start() {
    let input = "$abc";
    no_expansion(&input);
}

#[test]
fn test_no_expansion_with_multiple_dollar_signs() {
    let input = "a$b$c";
    no_expansion(&input);
}

#[test]
fn test_no_expansion_with_dollar_sign_in_middle() {
    let input = "no dollar sign at all $ end";
    no_expansion(&input);
}

