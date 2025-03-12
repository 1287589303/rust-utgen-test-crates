// Answer 0

#[test]
fn test_fmt_empty_iter() {
    let buckets: Vec<Bucket<i32, String>> = Vec::new();
    let iter = Iter { iter: buckets.iter() };
    let mut formatter = fmt::Formatter::new();
    let _ = iter.fmt(&mut formatter);
}

#[test]
fn test_fmt_single_bucket() {
    let buckets = vec![Bucket { hash: 0, key: 1, value: "one".to_string() }];
    let iter = Iter { iter: buckets.iter() };
    let mut formatter = fmt::Formatter::new();
    let _ = iter.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_buckets() {
    let buckets = vec![
        Bucket { hash: 1, key: 1, value: "one".to_string() },
        Bucket { hash: 2, key: 2, value: "two".to_string() },
        Bucket { hash: 3, key: 3, value: "three".to_string() },
    ];
    let iter = Iter { iter: buckets.iter() };
    let mut formatter = fmt::Formatter::new();
    let _ = iter.fmt(&mut formatter);
}

