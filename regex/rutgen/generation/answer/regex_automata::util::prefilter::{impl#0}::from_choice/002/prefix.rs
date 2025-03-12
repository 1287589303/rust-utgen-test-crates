// Answer 0

#[test]
fn test_from_choice_byteset_zero_length() {
    let choice = Choice::ByteSet(ByteSet([true; 256]));
    let max_needle_len = 0;
    let result = Prefilter::from_choice(choice, max_needle_len);
}

#[test]
fn test_from_choice_byteset_half_length() {
    let choice = Choice::ByteSet(ByteSet([true; 256]));
    let max_needle_len = 128;
    let result = Prefilter::from_choice(choice, max_needle_len);
}

#[test]
fn test_from_choice_byteset_full_length() {
    let choice = Choice::ByteSet(ByteSet([true; 256]));
    let max_needle_len = 256;
    let result = Prefilter::from_choice(choice, max_needle_len);
}

