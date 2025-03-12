// Answer 0

#[test]
fn test_len_empty_iter() {
    let buckets: Vec<Bucket<u32, u32>> = vec![];
    let keys = Keys { iter: buckets.iter() };
    let length = keys.len();
}

#[test]
fn test_len_single_element_iter() {
    let buckets = vec![Bucket { hash: 0, key: 1, value: 2 }];
    let keys = Keys { iter: buckets.iter() };
    let length = keys.len();
}

#[test]
fn test_len_multiple_elements_iter() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 2 },
        Bucket { hash: 1, key: 3, value: 4 },
        Bucket { hash: 2, key: 5, value: 6 },
    ];
    let keys = Keys { iter: buckets.iter() };
    let length = keys.len();
}

