// Answer 0

#[test]
fn test_float_valid_input_with_ident() {
    let cursor = Cursor {
        rest: "3.14a",
        off: 0,
    };
    let _ = float(cursor);
}

#[test]
fn test_float_valid_input_with_ident_and_underscore() {
    let cursor = Cursor {
        rest: "2.718zeta",
        off: 0,
    };
    let _ = float(cursor);
}

#[test]
fn test_float_with_exponent_and_ident() {
    let cursor = Cursor {
        rest: "1.5e10b",
        off: 0,
    };
    let _ = float(cursor);
}

#[test]
fn test_float_with_exponent_negative_and_ident() {
    let cursor = Cursor {
        rest: "7.22e-5x",
        off: 0,
    };
    let _ = float(cursor);
}

#[test]
fn test_float_with_trailing_underscore_ident() {
    let cursor = Cursor {
        rest: "4.0_abc",
        off: 0,
    };
    let _ = float(cursor);
}

