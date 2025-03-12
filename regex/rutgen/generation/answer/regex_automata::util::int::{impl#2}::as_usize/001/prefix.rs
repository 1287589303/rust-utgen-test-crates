// Answer 0

#[test]
fn test_as_usize_zero() {
    let value: u32 = 0;
    let result = value.as_usize();
}

#[test]
fn test_as_usize_max() {
    let value: u32 = 4_294_967_295;
    let result = value.as_usize();
}

#[test]
fn test_as_usize_mid() {
    let value: u32 = 2_147_483_648;
    let result = value.as_usize();
}

#[test]
fn test_as_usize_small() {
    let value: u32 = 1;
    let result = value.as_usize();
}

#[test]
fn test_as_usize_large() {
    let value: u32 = 4_000_000_000;
    let result = value.as_usize();
}

#[test]
#[should_panic]
fn test_as_usize_overflow() {
    let value: u32 = 4_294_967_296; // Expected to panic in debug builds only
    let result = value.as_usize();
}

