// Answer 0

#[test]
fn test_drain_valid_range() {
    let mut core = IndexMapCore::<usize, usize>::new();
    core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    core.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 200 });
    core.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 300 });
    let _drained = core.drain(0..2);
}

#[test]
fn test_drain_same_start_end() {
    let mut core = IndexMapCore::<usize, usize>::new();
    core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    let _drained = core.drain(1..1);
}

#[test]
fn test_drain_unbounded() {
    let mut core = IndexMapCore::<usize, usize>::new();
    core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    let _drained = core.drain(..);
}

#[test]
#[should_panic]
fn test_drain_out_of_bounds_negative_start() {
    let mut core = IndexMapCore::<usize, usize>::new();
    let _drained = core.drain(-1..1);
}

#[test]
#[should_panic]
fn test_drain_out_of_bounds_start_greater_than_length() {
    let mut core = IndexMapCore::<usize, usize>::new();
    core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    let _drained = core.drain(2..3);
}

#[test]
#[should_panic]
fn test_drain_start_greater_than_end() {
    let mut core = IndexMapCore::<usize, usize>::new();
    core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    let _drained = core.drain(1..0);
}

#[test]
fn test_drain_max_size() {
    let mut core = IndexMapCore::<usize, usize>::new();
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i as usize });
    }
    let _drained = core.drain(0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

