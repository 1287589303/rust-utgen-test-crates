// Answer 0

#[test]
fn test_index_mut_valid_index_0() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    let _value = map.index_mut(0);
}

#[test]
fn test_index_mut_valid_index_1() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let _value = map.index_mut(1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_mut_invalid_index_equal_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    let _value = map.index_mut(1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_mut_invalid_index_greater_than_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    let _value = map.index_mut(2);
}

