// Answer 0

#[test]
fn test_chunks_vectored_empty_self_and_dst() {
    let buf: VecDeque<u8> = VecDeque::new();
    let mut dst = vec![io::IoSlice::new(&[]); 2];
    let result = buf.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_empty_self_non_empty_dst() {
    let buf: VecDeque<u8> = VecDeque::new();
    let mut dst = vec![io::IoSlice::new(&[0u8]); 2];
    let result = buf.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_non_empty_self_empty_dst() {
    let buf: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    let mut dst = vec![];
    let result = buf.chunks_vectored(&mut dst);
}

