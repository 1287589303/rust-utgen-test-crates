// Answer 0

#[test]
fn test_bernoulli_new_smallest_valid_probability() {
    let p = 2.0_f64.powi(-64);
    let _ = Bernoulli::new(p);
}

#[test]
fn test_bernoulli_new_largest_valid_probability() {
    let p = 1.0_f64 - 2.0_f64.powi(-64);
    let _ = Bernoulli::new(p);
}

#[test]
fn test_bernoulli_new_middle_valid_probability() {
    let p = 0.5;
    let _ = Bernoulli::new(p);
}

#[test]
fn test_bernoulli_new_near_zero_probability() {
    let p = 0.0_f64 + 2.0_f64.powi(-64);
    let _ = Bernoulli::new(p);
}

#[test]
fn test_bernoulli_new_near_one_probability() {
    let p = 1.0_f64 - 2.0_f64.powi(-63);
    let _ = Bernoulli::new(p);
}

