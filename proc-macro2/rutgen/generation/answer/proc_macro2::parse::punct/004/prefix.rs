// Answer 0

#[test]
fn test_punct_valid_case_joint_spacing() {
    let input_cursor = Cursor {
        rest: "!!valid_ident",
        off: 0,
    };

    let result = punct(input_cursor);
}

#[test]
fn test_punct_another_valid_case_joint_spacing() {
    let input_cursor = Cursor {
        rest: "+another_ident",
        off: 0,
    };

    let result = punct(input_cursor);
}

