// Answer 0

#[test]
fn test_next_u64_valid() {
    let mut rng = StdRng(Rng::from_entropy());
    let value = rng.next_u64();
}

#[test]
fn test_next_u64_boundary_min() {
    let mut rng = StdRng(Rng::from_entropy());
    let mut values = Vec::new();
    for _ in 0..10 {
        values.push(rng.next_u64());
    }
    let min_value = *values.iter().min().unwrap();
}

#[test]
fn test_next_u64_boundary_max() {
    let mut rng = StdRng(Rng::from_entropy());
    let mut values = Vec::new();
    for _ in 0..10 {
        values.push(rng.next_u64());
    }
    let max_value = *values.iter().max().unwrap();
}

