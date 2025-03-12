// Answer 0

#[test]
fn test_script_valid_canonical_name() {
    let canonical_name: &'static str = "Latin"; // Assuming "Latin" is a valid script
    let _result = script(canonical_name);
}

#[test]
fn test_script_non_existing_script_name() {
    let canonical_name: &'static str = "NonExistingScript"; // Non-existing script
    let _result = script(canonical_name);
}

#[test]
fn test_script_empty_string() {
    let canonical_name: &'static str = ""; // Empty string
    let _result = script(canonical_name);
}

#[test]
fn test_script_exceed_length_limit() {
    let canonical_name: &'static str = "ThisScriptNameIsWayTooLongForTheLimit"; // Exceeds length limits
    let _result = script(canonical_name);
}

