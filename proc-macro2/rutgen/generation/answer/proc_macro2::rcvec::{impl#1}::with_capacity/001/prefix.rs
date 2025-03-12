// Answer 0

#[test]
fn test_with_capacity_zero() {
    let cap = 0;
    let builder = RcVecBuilder::with_capacity(cap);
}

#[test]
fn test_with_capacity_one() {
    let cap = 1;
    let builder = RcVecBuilder::with_capacity(cap);
}

#[test]
fn test_with_capacity_hundred() {
    let cap = 100;
    let builder = RcVecBuilder::with_capacity(cap);
}

#[test]
fn test_with_capacity_one_thousand_twenty_four() {
    let cap = 1024;
    let builder = RcVecBuilder::with_capacity(cap);
}

#[test]
fn test_with_capacity_max_usize() {
    let cap = usize::MAX;
    let builder = RcVecBuilder::with_capacity(cap);
}

