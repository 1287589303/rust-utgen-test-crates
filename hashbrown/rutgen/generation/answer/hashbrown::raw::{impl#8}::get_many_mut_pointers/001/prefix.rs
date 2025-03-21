// Answer 0

#[test]
fn test_get_many_mut_pointers_empty() {
    let raw_table: RawTable<i32> = RawTable::new_in(Global);
    let hashes: [u64; 3] = [1, 2, 3];
    let eq = |_: usize, _: &i32| false;
    
    unsafe {
        let result = raw_table.get_many_mut_pointers::<3>(hashes, eq);
        let expected: [Option<NonNull<i32>>; 3] = [None, None, None];
        // Here the test would validate result against expected, omitted as per instructions.
    }
}

#[test]
fn test_get_many_mut_pointers_filled() {
    let mut raw_table: RawTable<i32> = RawTable::with_capacity_in(5, Global);
    let hashes: [u64; 3] = [1, 2, 3];
    let eq = |index: usize, value: &i32| *value == (index as i32 + 1); // Assume values are 1, 2, 3

    // Fill the RawTable
    unsafe {
        for (i, hash) in hashes.iter().enumerate() {
            raw_table.insert(*hash, (i + 1) as i32, |v| *v as u64);
        }
    }

    unsafe {
        let result = raw_table.get_many_mut_pointers::<3>(hashes, eq);
        // Here the test would validate result against expected output, omitted as per instructions.
    }
}

#[test]
fn test_get_many_mut_pointers_partial_fill() {
    let mut raw_table: RawTable<i32> = RawTable::with_capacity_in(5, Global);
    let hashes: [u64; 4] = [1, 2, 3, 4];
    let eq = |index: usize, value: &i32| *value == (index as i32 + 1); // Assume values are 1 and 2

    // Fill only partially the RawTable
    unsafe {
        raw_table.insert(1, 1, |v| *v as u64);
        raw_table.insert(2, 2, |v| *v as u64);
    }

    unsafe {
        let result = raw_table.get_many_mut_pointers::<4>(hashes, eq);
        // Here the test would validate result against expected output, omitted as per instructions.
    }
}

#[test]
fn test_get_many_mut_pointers_beyond_capacity() {
    let mut raw_table: RawTable<i32> = RawTable::with_capacity_in(5, Global);
    let hashes: [u64; 6] = [1, 2, 3, 4, 5, 6];
    let eq = |_: usize, _: &i32| false;

    unsafe {
        let result = raw_table.get_many_mut_pointers::<6>(hashes, eq);
        let expected: [Option<NonNull<i32>>; 6] = [None, None, None, None, None, None];
        // Here the test would validate result against expected, omitted as per instructions.
    }
}

