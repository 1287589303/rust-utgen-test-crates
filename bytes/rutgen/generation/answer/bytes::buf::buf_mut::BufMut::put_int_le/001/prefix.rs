// Answer 0

#[test]
fn test_put_int_le_one_byte() {
    let mut buf = Vec::with_capacity(1);
    buf.put_int_le(-1, 1);
}

#[test]
fn test_put_int_le_two_bytes() {
    let mut buf = Vec::with_capacity(2);
    buf.put_int_le(-256, 2);
}

#[test]
fn test_put_int_le_three_bytes() {
    let mut buf = Vec::with_capacity(3);
    buf.put_int_le(-65793, 3);
}

#[test]
fn test_put_int_le_four_bytes() {
    let mut buf = Vec::with_capacity(4);
    buf.put_int_le(-16909060, 4);
}

#[test]
fn test_put_int_le_five_bytes() {
    let mut buf = Vec::with_capacity(5);
    buf.put_int_le(-280602721, 5);
}

#[test]
fn test_put_int_le_six_bytes() {
    let mut buf = Vec::with_capacity(6);
    buf.put_int_le(-15039414, 6);
}

#[test]
fn test_put_int_le_seven_bytes() {
    let mut buf = Vec::with_capacity(7);
    buf.put_int_le(-12297829382473, 7);
}

#[test]
fn test_put_int_le_eight_bytes() {
    let mut buf = Vec::with_capacity(8);
    buf.put_int_le(-9223372036854775808, 8);
}

