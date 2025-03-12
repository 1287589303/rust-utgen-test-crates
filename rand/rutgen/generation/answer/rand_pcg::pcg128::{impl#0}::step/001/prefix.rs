// Answer 0

#[test]
fn test_step_minimum_state_increment() {
    let mut rng = Lcg128Xsl64::new(0, 1);
    rng.step();
}

#[test]
fn test_step_maximum_state_increment() {
    let mut rng = Lcg128Xsl64::new(u128::MAX, u128::MAX);
    rng.step();
}

#[test]
fn test_step_middle_state_increment() {
    let mut rng = Lcg128Xsl64::new(u128::MAX / 2, u128::MAX / 2);
    rng.step();
}

#[test]
fn test_step_pcg_zero_increment() {
    let mut rng = Lcg128Xsl64::new(1, 0);
    rng.step();
}

#[test]
fn test_step_boundary_increment() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.step();
}

