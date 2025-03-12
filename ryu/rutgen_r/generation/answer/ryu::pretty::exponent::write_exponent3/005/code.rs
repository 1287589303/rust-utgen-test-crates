// Answer 0

#[test]
fn test_write_exponent3_k_100() {
    const MAX_SIZE: usize = 5;
    let mut output: [u8; MAX_SIZE] = [0; MAX_SIZE];
    let result_ptr = output.as_mut_ptr();
    let k: isize = 100;

    let len = unsafe { write_exponent3(k, result_ptr) };

    assert_eq!(len, 3);
    assert_eq!(&output[..len], b"100");
}

