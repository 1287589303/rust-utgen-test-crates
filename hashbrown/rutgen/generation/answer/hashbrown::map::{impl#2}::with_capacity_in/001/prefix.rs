// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(0, &bump);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(1, &bump);
}

#[test]
fn test_with_capacity_in_medium_capacity() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(10, &bump);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(1000, &bump);
}

