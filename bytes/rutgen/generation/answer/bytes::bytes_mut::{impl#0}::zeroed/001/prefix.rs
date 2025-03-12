// Answer 0

#[test]
fn test_zeroed_with_len_zero() {
    let zeros = BytesMut::zeroed(0);
    let _len = zeros.len();
    let _capacity = zeros.capacity();
}

#[test]
fn test_zeroed_with_len_one() {
    let zeros = BytesMut::zeroed(1);
    let _len = zeros.len();
    let _capacity = zeros.capacity();
}

#[test]
fn test_zeroed_with_len_ten() {
    let zeros = BytesMut::zeroed(10);
    let _len = zeros.len();
    let _capacity = zeros.capacity();
}

#[test]
fn test_zeroed_with_len_forty_two() {
    let zeros = BytesMut::zeroed(42);
    let _len = zeros.len();
    let _capacity = zeros.capacity();
}

#[test]
fn test_zeroed_with_len_one_k() {
    let zeros = BytesMut::zeroed(1024);
    let _len = zeros.len();
    let _capacity = zeros.capacity();
}

#[test]
#[should_panic]
fn test_zeroed_with_len_max() {
    let _zeros = BytesMut::zeroed(usize::MAX);
}

