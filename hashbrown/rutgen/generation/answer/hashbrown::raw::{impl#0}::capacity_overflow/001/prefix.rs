// Answer 0

#[test]
#[should_panic]
fn test_capacity_overflow_infallible() {
    let fallibility = Fallibility::Infallible;
    fallibility.capacity_overflow();
}

