// Answer 0

#[test]
fn test_string_replacement_no_capture_references() {
    let mut dst = String::new();
    let replacement = "This is a test string.";
    
    let append = |_, _: &mut String| {};
    let name_to_index = |_| None;

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_replacement_no_capture_references_with_leading_text() {
    let mut dst = String::new();
    let replacement = "Leading text followed by a sentence.";
    
    let append = |_, _: &mut String| {};
    let name_to_index = |_| None;

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_replacement_no_capture_references_with_trailing_text() {
    let mut dst = String::new();
    let replacement = "A sentence followed by trailing text.";
    
    let append = |_, _: &mut String| {};
    let name_to_index = |_| None;

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_replacement_no_capture_references_with_only_text() {
    let mut dst = String::new();
    let replacement = "Just some plain text.";
    
    let append = |_, _: &mut String| {};
    let name_to_index = |_| None;

    string(replacement, append, name_to_index, &mut dst);
}

