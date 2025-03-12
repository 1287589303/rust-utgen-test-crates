// Answer 0

#[test]
fn test_last_mut_non_empty_u() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(BufStruct, &b"test"[..]);
    let last_mut_ref = chain.last_mut();
}

#[test]
fn test_last_mut_non_empty_t() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(&b"test"[..], BufStruct);
    let last_mut_ref = chain.last_mut();
}

#[test]
fn test_last_mut_empty_t_non_empty_u() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(&b""[..], &b"non-empty"[..]);
    let last_mut_ref = chain.last_mut();
}

#[test]
fn test_last_mut_non_empty_t_empty_u() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(&b"non-empty"[..], &b""[..]);
    let last_mut_ref = chain.last_mut();
}

#[test]
fn test_last_mut_empty_t_empty_u() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(&b""[..], &b""[..]);
    let last_mut_ref = chain.last_mut();
}

#[test]
fn test_last_mut_multiple_chars() {
    struct BufStruct;
    impl Buf for BufStruct {}

    let mut chain = Chain::new(&b"long string"[..], &b"another long string"[..]);
    let last_mut_ref = chain.last_mut();
}

