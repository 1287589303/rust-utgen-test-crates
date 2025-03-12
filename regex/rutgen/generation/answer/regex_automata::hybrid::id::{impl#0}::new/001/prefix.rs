// Answer 0

#[test]
fn test_new_id_beyond_max_1() {
    let id = LazyStateID::MAX + 1;
    let _result = LazyStateID::new(id);
}

#[test]
fn test_new_id_beyond_max_2() {
    let id = LazyStateID::MAX + 1000;
    let _result = LazyStateID::new(id);
}

#[test]
fn test_new_id_beyond_max_3() {
    let id = u64::MAX as usize;
    let _result = LazyStateID::new(id);
}

