// Answer 0

#[test]
fn test_is_boundary_case_0() {
    let bytes = [0b1000_0000];
    let i = 0;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_1() {
    let bytes = [0b1100_0001];
    let i = 0;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_2() {
    let bytes = [0b1100_0000];
    let i = 0;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_3() {
    let bytes = [0b1111_1111];
    let i = 0;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_4() {
    let bytes = [0b1000_0000, 0b1100_0001, 0b1100_0000, 0b1111_1111];
    let i = 1;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_5() {
    let bytes = [0b1000_0000, 0b1100_0001, 0b1100_0000, 0b1111_1111];
    let i = 2;
    let _ = is_boundary(&bytes, i);
}

#[test]
fn test_is_boundary_case_6() {
    let bytes = [0b1000_0000, 0b1100_0001, 0b1100_0000, 0b1111_1111];
    let i = 3;
    let _ = is_boundary(&bytes, i);
}

