// Answer 0

#[test]
fn test_shift_insert_existing_key_move_to_end() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();

    map.shift_insert(len - 1, 'a', ());
}
#[test]
#[should_panic]
fn test_shift_insert_existing_key_invalid_move() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();

    map.shift_insert(len, 'a', ());
}
#[test]
fn test_shift_insert_new_key_at_end() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();

    map.shift_insert(len, '*', ());
}
#[test]
fn test_shift_insert_existing_key_move_up() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();

    map.shift_insert(len - 2, 'a', ());
}
#[test]
#[should_panic]
fn test_shift_insert_existing_key_move_invalid() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();

    map.shift_insert(len, 'z', ());
}

