// Answer 0

#[test]
fn test_step_with_minimal_state_and_increment() {
    let mut pcg = Lcg64Xsh32::new(0, 1);
    pcg.step();
}

#[test]
fn test_step_with_maximal_state_and_increment() {
    let mut pcg = Lcg64Xsh32::new(u64::MAX, u64::MAX);
    pcg.step();
}

#[test]
fn test_step_with_boundary_state_minimum_increment() {
    let mut pcg = Lcg64Xsh32::new(0, 2);
    pcg.step();
}

#[test]
fn test_step_with_boundary_state_maximum_increment() {
    let mut pcg = Lcg64Xsh32::new(u64::MAX - 1, 1);
    pcg.step();
}

#[test]
fn test_step_with_random_state_and_increment() {
    let mut pcg = Lcg64Xsh32::new(12345678901234567890, 987654321098765432);
    pcg.step();
}

#[test]
fn test_step_with_even_increment() {
    let mut pcg = Lcg64Xsh32::new(1, 2);
    pcg.step();
}

