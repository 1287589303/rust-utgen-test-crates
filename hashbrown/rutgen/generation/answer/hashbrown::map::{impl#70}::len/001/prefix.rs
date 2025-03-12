// Answer 0

#[test]
fn test_len_non_empty_map() {
    struct TestKey;
    struct TestValue;

    let mut map: crate::RawTable<TestKey, TestValue> = crate::RawTable::default();
    map.insert(TestKey, TestValue);

    let values_mut = ValuesMut {
        inner: IterMut {
            inner: map.iter_mut(),
            marker: PhantomData,
        },
    };

    let length = values_mut.len();
}

#[test]
fn test_len_empty_map() {
    struct TestKey;
    struct TestValue;

    let map: crate::RawTable<TestKey, TestValue> = crate::RawTable::default();

    let values_mut = ValuesMut {
        inner: IterMut {
            inner: map.iter_mut(),
            marker: PhantomData,
        },
    };

    let length = values_mut.len();
}

#[test]
fn test_len_max_capacity_map() {
    struct TestKey;
    struct TestValue;

    let mut map: crate::RawTable<TestKey, TestValue> = crate::RawTable::default();
    
    // Assuming a method to reach maximum capacity which would depend on system constraints:
    // Here we'll simulate this by inserting a number of entries, up to an arbitrary large limit.
    for _ in 0..1000 { // Adjust the limit based on practical tests for capacity
        map.insert(TestKey, TestValue);
    }

    let values_mut = ValuesMut {
        inner: IterMut {
            inner: map.iter_mut(),
            marker: PhantomData,
        },
    };

    let length = values_mut.len();
}

