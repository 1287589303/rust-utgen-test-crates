// Answer 0

#[test]
fn test_p_with_zero_p_int() {
    let bernoulli = Bernoulli { p_int: 0 };
    let _ = bernoulli.p();
}

#[test]
fn test_p_with_half_scale() {
    let bernoulli = Bernoulli { p_int: (u64::MAX >> 1) }; 
    let _ = bernoulli.p();
}

#[test]
fn test_p_with_max_minus_one() {
    let bernoulli = Bernoulli { p_int: u64::MAX - 1 };
    let _ = bernoulli.p();
}

#[test]
fn test_p_with_small_value() {
    let bernoulli = Bernoulli { p_int: 1 };
    let _ = bernoulli.p();
}

#[test]
fn test_p_with_large_value() {
    let bernoulli = Bernoulli { p_int: u64::MAX / 10 }; 
    let _ = bernoulli.p();
}

