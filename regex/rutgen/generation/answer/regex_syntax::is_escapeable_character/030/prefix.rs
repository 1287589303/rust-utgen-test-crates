// Answer 0

#[test]
fn test_escapeable_character_special_characters() {
    let inputs = vec!['#', '%', '&', '-', '/', '!', '"', '@', '^', '=', '+', '*', '?', '{', '}', '|', '\\'];
    for &c in &inputs {
        let _result = is_escapeable_character(c);
    }
}

#[test]
fn test_escapeable_character_boundary() {
    let inputs = vec![' ', '\'', ':', ';', ',', '.', '~', '`', '(', ')', '[', ']', '^', '$'];
    for &c in &inputs {
        let _result = is_escapeable_character(c);
    }
}

