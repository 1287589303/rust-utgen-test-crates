// Answer 0

#[test]
fn test_matches_with_boundary_condition_failure() {
    let utf8_seq = Utf8Sequence::Three([
        Utf8Range::new(0x30, 0x39), // '0' to '9'
        Utf8Range::new(0x41, 0x5A), // 'A' to 'Z'
        Utf8Range::new(0x61, 0x7A), // 'a' to 'z'
    ]);

    let bytes = [0x31, 0x43, 0x7B]; // '1', 'C', and '{' (not in range)
    
    utf8_seq.matches(&bytes);
}

#[test]
fn test_matches_with_non_matching_middle() {
    let utf8_seq = Utf8Sequence::Four([
        Utf8Range::new(0x61, 0x7A), // 'a' to 'z'
        Utf8Range::new(0x30, 0x39), // '0' to '9'
        Utf8Range::new(0x41, 0x5A), // 'A' to 'Z'
        Utf8Range::new(0x20, 0x7E), // space to '~'
    ]);

    let bytes = [0x61, 0x35, 0x40, 0x7F]; // 'a', '5', '@', and DEL (not matched)
    
    utf8_seq.matches(&bytes);
}

#[test]
fn test_matches_with_non_matching_last() {
    let utf8_seq = Utf8Sequence::Two([
        Utf8Range::new(0x31, 0x39), // '1' to '9'
        Utf8Range::new(0x41, 0x4A), // 'A' to 'J'
    ]);

    let bytes = [0x39, 0x4B]; // '9' and 'K' (not in range)
    
    utf8_seq.matches(&bytes);
}

