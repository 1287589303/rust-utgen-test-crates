// Answer 0

#[test]
fn test_move_index_valid_bounds() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: 1, key: 0, value: 100 });
    index_map.entries.push(Bucket { hash: 2, key: 1, value: 200 });
    index_map.entries.push(Bucket { hash: 3, key: 2, value: 300 });
    index_map.entries.push(Bucket { hash: 4, key: 3, value: 400 });
    
    index_map.move_index(0, 2);
}

#[test]
fn test_move_index_swapping_non_adjacent() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: 1, key: 0, value: 100 });
    index_map.entries.push(Bucket { hash: 2, key: 1, value: 200 });
    index_map.entries.push(Bucket { hash: 3, key: 2, value: 300 });
    
    index_map.move_index(0, 2);
}

#[test]
fn test_move_index_swapping_adjacent() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: 1, key: 0, value: 100 });
    index_map.entries.push(Bucket { hash: 2, key: 1, value: 200 });
    
    index_map.move_index(0, 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: 1, key: 0, value: 100 });
    
    index_map.move_index(1, 0);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: 1, key: 0, value: 100 });
    
    index_map.move_index(0, 1);
}

