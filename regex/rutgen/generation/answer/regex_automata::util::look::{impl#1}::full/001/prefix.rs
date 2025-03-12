// Answer 0

#[test]
fn test_look_set_full() {
    let result = LookSet::full();
}

#[test]
fn test_look_set_full_representation() {
    let result = LookSet::full();
    assert_eq!(result.bits, !0);
}

