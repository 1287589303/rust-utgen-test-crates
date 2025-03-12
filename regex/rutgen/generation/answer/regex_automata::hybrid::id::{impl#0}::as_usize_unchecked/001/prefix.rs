// Answer 0

#[test]
fn test_as_usize_unchecked_valid() {
    let id_0 = LazyStateID::new_unchecked(0);
    let id_max = LazyStateID::new_unchecked(LazyStateID::MAX as usize);
    
    let _val_0 = id_0.as_usize_unchecked();
    let _val_max = id_max.as_usize_unchecked();
}

#[test]
fn test_as_usize_unchecked_overflow() {
    let id_above_max = LazyStateID::new_unchecked(LazyStateID::MAX as usize + 1);
    
    let _val_above_max = id_above_max.as_usize_unchecked();
}

