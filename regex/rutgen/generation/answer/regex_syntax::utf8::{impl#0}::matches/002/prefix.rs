// Answer 0

#[test]
fn test_matches_one_byte_range() {
    let seq = Utf8Sequence::One(Utf8Range::new(0x61, 0x61)); // 'a'
    let bytes = &[0x61]; // Matches 'a'
    let result = seq.matches(bytes);
}

#[test]
fn test_matches_two_byte_ranges() {
    let seq = Utf8Sequence::Two([
        Utf8Range::new(0xC2, 0xC2), // 'Â'
        Utf8Range::new(0xA0, 0xA0), // ' '
    ]);
    let bytes = &[0xC2, 0xA0]; // Matches 'Â '
    let result = seq.matches(bytes);
}

#[test]
fn test_matches_three_byte_ranges() {
    let seq = Utf8Sequence::Three([
        Utf8Range::new(0xE2, 0xE2), // 'â'
        Utf8Range::new(0x82, 0x82), // '‚'
        Utf8Range::new(0xAC, 0xAC), // '¬'
    ]);
    let bytes = &[0xE2, 0x82, 0xAC]; // Matches 'â‚¬'
    let result = seq.matches(bytes);
}

#[test]
fn test_matches_four_byte_ranges() {
    let seq = Utf8Sequence::Four([
        Utf8Range::new(0xF0, 0xF0), // 'ð'
        Utf8Range::new(0x9F, 0x9F), // 'Ÿ'
        Utf8Range::new(0x92, 0x92), // '’'
        Utf8Range::new(0xA8, 0xA8), // '¨'
    ]);
    let bytes = &[0xF0, 0x9F, 0x92, 0xA8]; // Matches 'ðŸ’¨'
    let result = seq.matches(bytes);
}

