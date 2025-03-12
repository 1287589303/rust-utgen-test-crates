// Answer 0

#[test]
fn test_shift_insert_existing_key_move() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(10, 'a', ());
    let occupied_index_of_a = map.get_index_of(&'a').unwrap();
    assert_eq!(occupied_index_of_a, 10);
    assert!(old_value.is_some());
}

#[test]
fn test_shift_insert_another_existing_key_move() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(5, 'f', ());
    let occupied_index_of_f = map.get_index_of(&'f').unwrap();
    assert_eq!(occupied_index_of_f, 5);
    assert!(old_value.is_some());
}

#[test]
fn test_shift_insert_edge_case_move() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(0, 'b', ());
    let occupied_index_of_b = map.get_index_of(&'b').unwrap();
    assert_eq!(occupied_index_of_b, 0);
    assert!(old_value.is_some());
}

