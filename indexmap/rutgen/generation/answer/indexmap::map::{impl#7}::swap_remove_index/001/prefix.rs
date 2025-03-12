// Answer 0

#[test]
fn test_swap_remove_index_valid_index_zero() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    map.insert(TestK(1), TestV(10));
    let result = map.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_valid_index_last() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    map.insert(TestK(1), TestV(10));
    map.insert(TestK(2), TestV(20));
    let result = map.swap_remove_index(map.len() - 1);
}

#[test]
#[should_panic]
fn test_swap_remove_index_invalid_index_negative() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    let result = map.swap_remove_index(usize::MAX);
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_too_large() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    map.insert(TestK(1), TestV(10));
    let result = map.swap_remove_index(2);
}

#[test]
fn test_swap_remove_index_multiple_elements() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    map.insert(TestK(1), TestV(10));
    map.insert(TestK(2), TestV(20));
    map.insert(TestK(3), TestV(30));
    let result = map.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_empty() {
    struct TestK(u32);
    struct TestV(u32);
    let mut map = IndexMap::<TestK, TestV, RandomState>::new();
    let result = map.swap_remove_index(0);
}

