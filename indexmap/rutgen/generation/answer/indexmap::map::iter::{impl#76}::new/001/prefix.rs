// Answer 0

#[test]
fn test_splice_new_valid_range() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);

    let range = 0..2;
    let replace_with = vec![(100, 100), (200, 200)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

#[test]
fn test_splice_new_empty_map() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();

    let range = 0..0;
    let replace_with = vec![(100, 100)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

#[test]
fn test_splice_new_full_range() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);

    let range = 0..3;
    let replace_with = vec![(100, 100), (200, 200), (300, 300)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

#[test]
fn test_splice_new_single_element() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(0, 10);
    
    let range = 0..1;
    let replace_with = vec![(100, 100)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

#[test]
fn test_splice_new_out_of_bounds_range() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);

    let range = 2..4;
    let replace_with = vec![(100, 100)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

#[test]
fn test_splice_new_range_with_replacements() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);

    let range = 1..2;
    let replace_with = vec![(100, 100)].into_iter();

    let splice = Splice::new(&mut map, range, replace_with);
}

