// Answer 0

#[test]
fn test_insert_with_existing_literal_and_new_byte() {
    let mut trie = PreferenceTrie {
        states: vec![],
        matches: vec![],
        next_literal_index: 1,
    };
    
    // First insert to create an existing literal
    let first_literal = b"hello";
    let _ = trie.insert(first_literal).unwrap();

    // Now insert a new byte after the existing literal
    let new_byte = b"h";
    let result = trie.insert(&[104, 101, 108, 108, 111, 120]); // "hello" + 'x'

    // Ensure the result is Ok with the literal index
    let _ = result.unwrap();
}

#[test]
fn test_insert_with_existing_literal_and_unique_byte() {
    let mut trie = PreferenceTrie {
        states: vec![],
        matches: vec![],
        next_literal_index: 1,
    };
    
    // Insert first literal
    let first_literal = b"world";
    let _ = trie.insert(first_literal).unwrap();

    // Now insert a unique byte after an existing literal
    let new_byte = b"w";
    let result = trie.insert(&[119, 111, 114, 108, 100, 121]); // "world" + 'y'
    
    // Ensure the result is Ok with the literal index
    let _ = result.unwrap();
}

#[test]
fn test_insert_with_existing_match_state_and_new_state() {
    let mut trie = PreferenceTrie {
        states: vec![],
        matches: vec![],
        next_literal_index: 1,
    };

    // Insert first literal to ensure state exists
    let first_literal = b"test";
    let _ = trie.insert(first_literal).unwrap();

    // Inserting prefix that matches 'test' but adds unique byte 'a'
    let result = trie.insert(b"testa");
    
    // Ensure the result is Ok with the literal index
    let _ = result.unwrap();
}

