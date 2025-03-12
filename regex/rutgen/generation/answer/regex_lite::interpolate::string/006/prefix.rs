// Answer 0

#[test]
fn test_string_with_valid_capture_ref() {
    let mut dst = String::new();
    let replacement = "abc$def${name}ghi";
    
    let mut append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("value0");
        } else if index == 1 {
            dst.push_str("value1");
        }
    };
    
    let name_to_index = |name: &str| {
        if name == "name" {
            Some(1) // Maps "name" to index 1
        } else {
            None
        }
    };
    
    string(replacement, append, name_to_index, &mut dst);
}

