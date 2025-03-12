// Answer 0

#[test]
fn test_next_u64_all_zeros() {
    let mut rng = Xoshiro256PlusPlus { s: [0, 0, 0, 0] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_incremental_values() {
    let mut rng = Xoshiro256PlusPlus { s: [1, 2, 3, 4] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_max_values() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MAX, u64::MAX, u64::MAX, u64::MAX] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_mixed_max() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MAX, u64::MAX, 0, 0] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_half_max() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MAX / 2, u64::MAX / 2, u64::MAX / 2, u64::MAX / 2] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_two_zeros() {
    let mut rng = Xoshiro256PlusPlus { s: [0, 0, u64::MAX, u64::MAX] };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_edge_case() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MAX - 1, u64::MAX, 0, 1] };
    let _ = rng.next_u64();
}

