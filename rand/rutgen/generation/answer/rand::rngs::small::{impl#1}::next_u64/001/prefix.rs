// Answer 0

#[test]
fn test_next_u64_zero_state() {
    let rng = Xoshiro256PlusPlus { s: [0, 0, 0, 0] };
    let mut small_rng = SmallRng(rng);
    small_rng.next_u64();
}

#[test]
fn test_next_u64_maximum_seed() {
    let rng = Xoshiro256PlusPlus { s: [u64::MAX, u64::MAX, u64::MAX, u64::MAX] };
    let mut small_rng = SmallRng(rng);
    small_rng.next_u64();
}

#[test]
fn test_next_u64_random_seed() {
    let rng = Xoshiro256PlusPlus { s: [1, 2, 3, 4] };
    let mut small_rng = SmallRng(rng);
    small_rng.next_u64();
}

#[test]
fn test_next_u64_alternate_seed() {
    let rng = Xoshiro256PlusPlus { s: [5, 10, 15, 20] };
    let mut small_rng = SmallRng(rng);
    small_rng.next_u64();
}

