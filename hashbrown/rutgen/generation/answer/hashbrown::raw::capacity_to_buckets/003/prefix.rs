// Answer 0

#[test]
fn test_capacity_to_buckets_cap_equal_four() {
    let cap: usize = 4;
    let result = capacity_to_buckets(cap);
}

#[test]
fn test_capacity_to_buckets_cap_less_than_four() {
    let cap: usize = 3;
    let result = capacity_to_buckets(cap);
}

#[test]
fn test_capacity_to_buckets_cap_equal_eight() {
    let cap: usize = 8;
    let result = capacity_to_buckets(cap);
}

#[test]
fn test_capacity_to_buckets_cap_seven() {
    let cap: usize = 7;
    let result = capacity_to_buckets(cap);
}

