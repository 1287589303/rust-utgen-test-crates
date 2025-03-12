// Answer 0

#[test]
fn test_shrink_to_fit_zero_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(0);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_one_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(1);
    map.insert(1, 2);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_ten_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
    for i in 0..10 {
        map.insert(i, i + 1);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_hundred_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    for i in 0..50 {
        map.insert(i, i + 1);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_thousand_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(1000);
    for i in 0..500 {
        map.insert(i, i + 1);
    }
    map.shrink_to_fit();
}

