// Answer 0

#[test]
fn test_iter_match_pattern_ids_with_non_empty_pids() {
    struct TestRepr<'a>(&'a [u8]);

    let data: [u8; 20] = [0b00000001, 0b00000010, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0];
    let repr = TestRepr(&data);

    repr.iter_match_pattern_ids(|pid| {
        // Processing the pattern ID
    });
}

#[test]
fn test_iter_match_pattern_ids_with_empty_pids() {
    struct TestRepr<'a>(&'a [u8]);

    let data: [u8; 13] = [0b00000001, 0b00000010, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let repr = TestRepr(&data);

    repr.iter_match_pattern_ids(|pid| {
        // This should only receive PatternID::ZERO
    });
}

