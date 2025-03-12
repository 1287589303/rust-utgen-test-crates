// Answer 0

#[test]
fn test_next_u64_with_v_zero_a_nonzero() {
    let mut rng = StepRng { v: 0, a: 5 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_nonzero_a_zero() {
    let mut rng = StepRng { v: 5, a: 0 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_maximum_a_nonzero() {
    let mut rng = StepRng { v: u64::MAX, a: 3 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_zero_a_zero() {
    let mut rng = StepRng { v: 0, a: 0 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_maximum_a_zero() {
    let mut rng = StepRng { v: u64::MAX, a: 0 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_zero_a_maximum() {
    let mut rng = StepRng { v: 0, a: u64::MAX };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_equal_a_nonzero() {
    let mut rng = StepRng { v: 10, a: 10 };
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_with_v_and_a_both_maximum() {
    let mut rng = StepRng { v: u64::MAX, a: u64::MAX };
    let result = rng.next_u64();
}

