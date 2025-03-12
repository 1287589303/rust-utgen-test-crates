// Answer 0

#[test]
#[should_panic]
fn test_drain_iter_from_empty_table() {
    let table: RawTable<u64> = RawTable::new_in(Global);
    let iter = RawIter { iter: RawIterRange::new(), items: 0 };
    unsafe {
        table.drain_iter_from(iter);
    }
}

#[test]
#[should_panic]
fn test_drain_iter_from_invalid_iterator_length_greater_than_table() {
    let table: RawTable<u64> = RawTable::with_capacity_in(5, Global);
    let iter = RawIter { iter: RawIterRange::new(), items: 6 }; // items greater than table
    unsafe {
        table.drain_iter_from(iter);
    }
}

#[test]
fn test_drain_iter_from_valid_iterator() {
    let mut table: RawTable<u64> = RawTable::with_capacity_in(5, Global);
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        let iter = table.iter(); // getting a valid iterator
        table.drain_iter_from(iter); // should not panic
    }
}

#[test]
fn test_drain_iter_from_invalid_iterator_zero_elements() {
    let mut table: RawTable<u64> = RawTable::with_capacity_in(5, Global);
    let iter = RawIter { iter: RawIterRange::new(), items: 1 }; // items greater than actual elements
    unsafe {
        table.drain_iter_from(iter); // should not panic as this is a valid call
    }
}

