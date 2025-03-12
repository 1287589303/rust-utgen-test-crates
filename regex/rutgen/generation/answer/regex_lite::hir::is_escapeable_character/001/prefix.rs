// Answer 0

#[test]
fn test_is_escapeable_character_meta() {
    let meta_chars = vec!['\\', '.', '+', '*', '?', '(', ')', '|', '[', ']', '{', '}', '^', '$', '#', '&', '-', '~'];
    for c in meta_chars {
        let _result = is_escapeable_character(c);
    }
}

#[test]
fn test_is_escapeable_character_non_meta() {
    let non_meta_chars = vec!['!', '@', '%', ',', ':', ';', '"', '\'', '/', '=', '^', '`', '<', '>'];
    for c in non_meta_chars {
        let _result = is_escapeable_character(c);
    }
}

