// Answer 0

#[test]
pub(crate) fn test_special_new() {
    let special = Special::new();
}

#[test]
pub(crate) fn test_special_fields_default() {
    let special = Special::new();
    let expected = Special {
        max: DEAD,
        quit_id: DEAD,
        min_match: DEAD,
        max_match: DEAD,
        min_accel: DEAD,
        max_accel: DEAD,
        min_start: DEAD,
        max_start: DEAD,
    };
    assert_eq!(special, expected);
}

