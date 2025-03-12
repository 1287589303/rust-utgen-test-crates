// Answer 0

#[test]
fn test_digits_hex_lowercase() {
    let input = Cursor {
        rest: "abcdef",
        off: 0,
    };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_uppercase() {
    let input = Cursor {
        rest: "ABCDEF",
        off: 0,
    };
    let _ = digits(input);
}

#[test]
fn test_digits_mixed_case() {
    let input = Cursor {
        rest: "aBcDeF",
        off: 0,
    };
    let _ = digits(input);
}

#[test]
fn test_digits_with_underscore() {
    let input = Cursor {
        rest: "a_b_c_d_e_f",
        off: 0,
    };
    let _ = digits(input);
}

#[test]
fn test_digits_no_hex() {
    let input = Cursor {
        rest: "abc",
        off: 0,
    };
    let _ = digits(input);
}

