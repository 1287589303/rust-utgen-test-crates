// Answer 0

#[test]
fn test_next_u32_zero_state() {
    let mut rng = Xoshiro256PlusPlus { s: [0, 0, 0, 0] };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_max_state() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MAX, u64::MAX, u64::MAX, u64::MAX] };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_min_state() {
    let mut rng = Xoshiro256PlusPlus { s: [u64::MIN, u64::MIN, u64::MIN, u64::MIN] };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_boundary_values() {
    let mut rng = Xoshiro256PlusPlus { s: [0, 0, 0, 0] };
    for _ in 0..10 {
        let _ = rng.next_u32();
    }
    rng.s = [u64::MAX, u64::MAX, u64::MAX, u64::MAX];
    for _ in 0..10 {
        let _ = rng.next_u32();
    }
    rng.s = [u64::MIN, u64::MIN, u64::MIN, u64::MIN];
    for _ in 0..10 {
        let _ = rng.next_u32();
    }
}

