// Answer 0

#[test]
fn test_len_empty() {
    let vec: Vec<Bucket<i32, i32>> = Vec::new();
    let drain = vec.drain(..);
    let len = drain.len();
}

#[test]
fn test_len_non_empty() {
    let mut vec: Vec<Bucket<i32, i32>> = Vec::with_capacity(5);
    vec.push(Bucket { hash: 0, key: 1, value: 10 });
    vec.push(Bucket { hash: 1, key: 2, value: 20 });
    let drain = vec.drain(..);
    let len = drain.len();
}

#[test]
fn test_len_full_capacity() {
    let mut vec: Vec<Bucket<i32, i32>> = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(Bucket { hash: i, key: i, value: i * 10 });
    }
    let drain = vec.drain(..);
    let len = drain.len();
}

#[test]
fn test_len_after_draining_some() {
    let mut vec: Vec<Bucket<i32, i32>> = Vec::new();
    vec.push(Bucket { hash: 0, key: 1, value: 10 });
    vec.push(Bucket { hash: 1, key: 2, value: 20 });
    let mut drain = vec.drain(..);
    drain.next(); // Drain one item
    let len = drain.len();
}

