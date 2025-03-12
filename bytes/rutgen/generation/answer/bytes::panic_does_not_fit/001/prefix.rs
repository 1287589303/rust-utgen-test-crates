// Answer 0

#[test]
fn test_panic_does_not_fit_size_1() {
    let size: usize = 1;
    let nbytes: usize = 2; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_2() {
    let size: usize = 2;
    let nbytes: usize = 3; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_3() {
    let size: usize = 3;
    let nbytes: usize = 4; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_4() {
    let size: usize = 4;
    let nbytes: usize = 5; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_5() {
    let size: usize = 5;
    let nbytes: usize = 6; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_6() {
    let size: usize = 6;
    let nbytes: usize = 7; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_7() {
    let size: usize = 7;
    let nbytes: usize = 8; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_size_8() {
    let size: usize = 8;
    let nbytes: usize = 9; // size + 1
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_boundary_size_1_nbytes_16() {
    let size: usize = 1;
    let nbytes: usize = 16; // upper limit
    panic_does_not_fit(size, nbytes);
}

#[test]
fn test_panic_does_not_fit_boundary_size_8_nbytes_16() {
    let size: usize = 8;
    let nbytes: usize = 16; // upper limit
    panic_does_not_fit(size, nbytes);
}

