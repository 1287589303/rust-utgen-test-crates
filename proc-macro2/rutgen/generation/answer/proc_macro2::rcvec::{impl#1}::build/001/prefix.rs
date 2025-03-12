// Answer 0

#[test]
fn test_rcvecbuilder_build_zero_elements() {
    let builder = RcVecBuilder::<i32>::new();
    let rc_vec = builder.build();
}

#[test]
fn test_rcvecbuilder_build_single_element() {
    let mut builder = RcVecBuilder::<i32>::new();
    builder.push(1);
    let rc_vec = builder.build();
}

#[test]
fn test_rcvecbuilder_build_multiple_elements() {
    let mut builder = RcVecBuilder::<i32>::new();
    builder.push(1);
    builder.push(2);
    builder.push(3);
    let rc_vec = builder.build();
}

#[test]
fn test_rcvecbuilder_build_with_capacity_zero() {
    let builder = RcVecBuilder::<i32>::with_capacity(0);
    let rc_vec = builder.build();
}

#[test]
fn test_rcvecbuilder_build_with_capacity_non_zero() {
    let mut builder = RcVecBuilder::<i32>::with_capacity(3);
    builder.push(1);
    builder.push(2);
    builder.push(3);
    let rc_vec = builder.build();
}

#[test]
fn test_rcvecbuilder_build_exact_capacity() {
    let mut builder = RcVecBuilder::<i32>::with_capacity(3);
    builder.push(1);
    builder.push(2);
    builder.push(3);
    let rc_vec = builder.build();
}

