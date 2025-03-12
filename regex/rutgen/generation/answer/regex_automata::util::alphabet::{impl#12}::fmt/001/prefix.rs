// Answer 0

#[test]
fn test_fmt_with_some_bytes_contained() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(10);
    byte_set.add(50);
    byte_set.add(200);
    let _ = format!("{:?}", byte_set.bits);
}

#[test]
fn test_fmt_with_all_bytes_contained() {
    let mut byte_set = ByteSet::empty();
    for b in 0u8..=255 {
        byte_set.add(b);
    }
    let _ = format!("{:?}", byte_set.bits);
}

#[test]
fn test_fmt_with_no_bytes_contained() {
    let byte_set = ByteSet::empty();
    let _ = format!("{:?}", byte_set.bits);
}

#[test]
fn test_fmt_with_boundary_bytes() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
    byte_set.add(255);
    let _ = format!("{:?}", byte_set.bits);
}

#[test]
fn test_fmt_with_bytes_both_in_and_out() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(100);
    byte_set.add(150);
    let _ = format!("{:?}", byte_set.bits); 
}

