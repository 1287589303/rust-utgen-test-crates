// Answer 0

#[test]
fn test_match_len_with_is_match_true_and_has_pattern_ids_false() {
    let data = [0b00000001, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
    let repr = Repr(&data);
    repr.match_len();
}

