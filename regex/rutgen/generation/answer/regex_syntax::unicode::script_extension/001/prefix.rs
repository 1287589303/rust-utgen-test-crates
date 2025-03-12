// Answer 0

#[test]
fn test_script_extension_valid_extension() {
    let canonical_name = "Latin";
    let _result = script_extension(canonical_name);
}

#[test]
fn test_script_extension_empty_string() {
    let canonical_name = "";
    let _result = script_extension(canonical_name);
}

#[test]
fn test_script_extension_non_existent_extension() {
    let canonical_name = "NonExistentScript";
    let _result = script_extension(canonical_name);
}

#[test]
fn test_script_extension_longest_string() {
    let canonical_name = "ThisIsAVeryLongCanonicalScriptExtensionNameThatMightBeAtTheLimit";
    let _result = script_extension(canonical_name);
}

#[test]
#[should_panic]
fn test_script_extension_when_unicode_script_disabled() {
    let canonical_name = "Latin";
    let _result = script_extension(canonical_name);
}

