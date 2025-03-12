// Answer 0

#[test]
fn test_advance_exact_match() {
    let mut buf: &mut [u8] = &mut [1, 2, 3];
    buf.advance(buf.len());
}

#[test]
fn test_advance_greater() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4, 5];
    buf.advance(3);
}

#[test]
#[should_panic]
fn test_advance_underflow() {
    let mut buf: &mut [u8] = &mut [];
    buf.advance(1);
}

