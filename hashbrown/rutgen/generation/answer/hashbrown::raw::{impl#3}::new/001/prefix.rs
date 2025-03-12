// Answer 0

#[test]
fn test_new_with_float_type() {
    struct TestStruct;

    let layout = TableLayout::new::<f64>();
    let expected_size = std::mem::size_of::<f64>();
    assert_eq!(layout.size, expected_size);
    assert!(layout.ctrl_align > Group::WIDTH);
}

#[test]
fn test_new_with_double_pointer() {
    struct TestStruct;

    let layout = TableLayout::new::<*const f64>();
    let expected_size = std::mem::size_of::<*const f64>();
    assert_eq!(layout.size, expected_size);
    assert!(layout.ctrl_align > Group::WIDTH);
}

#[test]
fn test_new_with_large_array() {
    struct TestStruct;

    let layout = TableLayout::new::<[f64; 100]>();
    let expected_size = std::mem::size_of::<[f64; 100]>();
    assert_eq!(layout.size, expected_size);
    assert!(layout.ctrl_align > Group::WIDTH);
}

