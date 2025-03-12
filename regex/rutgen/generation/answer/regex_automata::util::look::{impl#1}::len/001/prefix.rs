// Answer 0

#[test]
fn test_len_empty() {
    let look_set = LookSet::empty();
    look_set.len();
}

#[test]
fn test_len_singleton_one() {
    let look_set = LookSet { bits: 1 };
    look_set.len();
}

#[test]
fn test_len_singleton_two() {
    let look_set = LookSet { bits: 2 };
    look_set.len();
}

#[test]
fn test_len_singleton_four() {
    let look_set = LookSet { bits: 4 };
    look_set.len();
}

#[test]
fn test_len_singleton_eight() {
    let look_set = LookSet { bits: 8 };
    look_set.len();
}

#[test]
fn test_len_multiple() {
    let look_set = LookSet { bits: 15 }; // 1 + 2 + 4 + 8
    look_set.len();
}

#[test]
fn test_len_full() {
    let look_set = LookSet { bits: 4294967295 }; // all bits set
    look_set.len();
}

