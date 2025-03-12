// Answer 0

#[test]
fn test_iter_hash_mut_with_existing_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| val.wrapping_mul(31) as u64; // Simple hash function

    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&1), 3, hasher);
    
    let mut iter = table.iter_hash_mut(hasher(&1));
    while let Some(val) = iter.next() {
        *val *= 2; // Mutate values
    }
}

#[test]
fn test_iter_hash_mut_with_different_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| val.wrapping_mul(31) as u64; // Simple hash function

    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&2), 5, hasher);
    
    let iter = table.iter_hash_mut(hasher(&10)); // Valid hash, with no matches
    let mut count = 0;
    for _ in iter {
        count += 1; // No mutation, should not iterate
    }
    assert_eq!(count, 0);
}

#[test]
fn test_iter_hash_mut_empty() {
    let mut table = HashTable::new_in(Global);
    let iter = table.iter_hash_mut(1); // Any hash, but table is empty
    let mut count = 0;
    for _ in iter {
        count += 1; // No mutation, should not iterate
    }
    assert_eq!(count, 0);
}

#[test]
fn test_iter_hash_mut_boundary_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| val.wrapping_mul(31) as u64; // Simple hash function

    table.insert_unique(hasher(&0), 4, hasher); // Boundary case with hash 0
    table.insert_unique(hasher(&1), 1, hasher);
    
    let mut iter = table.iter_hash_mut(hasher(&0));
    while let Some(val) = iter.next() {
        *val += 1; // Mutate value
    }
}

#[test]
fn test_iter_hash_mut_large_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| val.wrapping_mul(31) as u64;
    
    table.insert_unique(2_u64.pow(63), 10, hasher); // Using large hash value
    let mut iter = table.iter_hash_mut(2_u64.pow(63));
    while let Some(val) = iter.next() {
        *val *= 3; // Mutate value
    }
}

