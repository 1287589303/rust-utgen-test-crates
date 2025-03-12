// Answer 0

#[test]
fn test_next_u32_minimum_state() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_maximum_state() {
    let mut rng = Lcg64Xsh32::new(u64::MAX, 1);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_small_increment() {
    let mut rng = Lcg64Xsh32::new(1, 1);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_large_increment() {
    let mut rng = Lcg64Xsh32::new(1, u64::MAX);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_mid_range_state_and_increment() {
    let mut rng = Lcg64Xsh32::new(1 << 32, 1 << 32);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_boundary_delta() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    rng.advance(0);
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_delta_max() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    rng.advance(u64::MAX);
    let result = rng.next_u32();
}

