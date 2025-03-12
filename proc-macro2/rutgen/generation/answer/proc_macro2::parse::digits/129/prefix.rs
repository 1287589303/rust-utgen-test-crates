// Answer 0

#[test]
fn test_digits_base_octal_invalid() {
    let input = Cursor { rest: "0oabcdefgh" };
    let result = digits(input);
}

