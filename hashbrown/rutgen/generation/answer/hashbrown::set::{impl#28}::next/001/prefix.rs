// Answer 0

#[test]
fn test_next_non_empty_map() {
    let mut map: HashMap<i32, ()> = HashMap::new();
    map.insert(1, ());
    map.insert(2, ());

    let keys_iter = Keys { inner: Iter { iter: map.keys() } };
    let mut iter = keys_iter.iter;

    let _first = iter.next();
    let _second = iter.next();
}

#[test]
fn test_next_empty_map() {
    let mut map: HashMap<i32, ()> = HashMap::new();

    let keys_iter = Keys { inner: Iter { iter: map.keys() } };
    let mut iter = keys_iter.iter;

    let _result = iter.next();
}

#[test]
fn test_next_map_with_one_element() {
    let mut map: HashMap<i32, ()> = HashMap::new();
    map.insert(1, ());

    let keys_iter = Keys { inner: Iter { iter: map.keys() } };
    let mut iter = keys_iter.iter;

    let _first = iter.next();
    let _result_after_first = iter.next();
}

#[test]
fn test_next_map_with_ten_elements() {
    let mut map: HashMap<i32, ()> = HashMap::new();
    for i in 1..=10 {
        map.insert(i, ());
    }

    let keys_iter = Keys { inner: Iter { iter: map.keys() } };
    let mut iter = keys_iter.iter;

    for _ in 0..10 {
        let _key = iter.next();
    }
    let _result_after_ten = iter.next();
}

#[test]
fn test_next_map_with_hundred_elements() {
    let mut map: HashMap<i32, ()> = HashMap::new();
    for i in 1..=100 {
        map.insert(i, ());
    }

    let keys_iter = Keys { inner: Iter { iter: map.keys() } };
    let mut iter = keys_iter.iter;

    for _ in 0..100 {
        let _key = iter.next();
    }
    let _result_after_hundred = iter.next();
}

