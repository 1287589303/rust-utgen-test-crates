// Answer 0

#[test]
fn test_canonical_value_valid_case() {
    let vals: PropertyValues = &[
        ("Letter", "L"),
        ("Digit", "D"),
        ("Whitespace", "W"),
    ];
    let normalized_value = "Letter";
    let _result = canonical_value(vals, normalized_value);
}

#[test]
fn test_canonical_value_valid_case_digit() {
    let vals: PropertyValues = &[
        ("Letter", "L"),
        ("Digit", "D"),
        ("Whitespace", "W"),
    ];
    let normalized_value = "Digit";
    let _result = canonical_value(vals, normalized_value);
}

#[test]
fn test_canonical_value_not_found() {
    let vals: PropertyValues = &[
        ("Letter", "L"),
        ("Digit", "D"),
        ("Whitespace", "W"),
    ];
    let normalized_value = "Punctuation";
    let _result = canonical_value(vals, normalized_value);
}

#[test]
fn test_canonical_value_empty_string() {
    let vals: PropertyValues = &[
        ("Letter", "L"),
        ("Digit", "D"),
        ("Whitespace", "W"),
    ];
    let normalized_value = "";
    let _result = canonical_value(vals, normalized_value);
}

#[test]
fn test_canonical_value_boundary_case() {
    let vals: PropertyValues = &[
        ("", "Empty"),
        ("Char", "C"),
        ("Whitespace", "W"),
    ];
    let normalized_value = "Char";
    let _result = canonical_value(vals, normalized_value);
}

