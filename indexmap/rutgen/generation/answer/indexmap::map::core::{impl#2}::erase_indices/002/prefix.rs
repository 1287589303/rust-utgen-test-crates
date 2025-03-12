// Answer 0

#[test]
fn test_erase_indices_with_some_erased_1() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        let hash_value = HashValue(i as usize);
        map.entries.push(Bucket { hash: hash_value, key: i, value: i * 10 });
        map.indices.insert_unique(hash_value.get(), i, |_| unreachable!());
    }
    map.erase_indices(2, 5);
}

#[test]
fn test_erase_indices_with_some_erased_2() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        let hash_value = HashValue(i as usize);
        map.entries.push(Bucket { hash: hash_value, key: i, value: i * 20 });
        map.indices.insert_unique(hash_value.get(), i, |_| unreachable!());
    }
    map.erase_indices(1, 6);
}

#[test]
fn test_erase_indices_with_some_erased_3() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(12);
    for i in 0..12 {
        let hash_value = HashValue(i as usize);
        map.entries.push(Bucket { hash: hash_value, key: i, value: i * 30 });
        map.indices.insert_unique(hash_value.get(), i, |_| unreachable!());
    }
    map.erase_indices(3, 10);
}

#[test]
fn test_erase_indices_with_some_erased_4() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(15);
    for i in 0..15 {
        let hash_value = HashValue(i as usize);
        map.entries.push(Bucket { hash: hash_value, key: i, value: i * 40 });
        map.indices.insert_unique(hash_value.get(), i, |_| unreachable!());
    }
    map.erase_indices(0, 7);
}

#[test]
fn test_erase_indices_with_some_erased_5() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(20);
    for i in 0..20 {
        let hash_value = HashValue(i as usize);
        map.entries.push(Bucket { hash: hash_value, key: i, value: i * 50 });
        map.indices.insert_unique(hash_value.get(), i, |_| unreachable!());
    }
    map.erase_indices(4, 11);
}

