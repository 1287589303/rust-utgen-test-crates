// Answer 0

#[test]
fn test_writer_with_minimum_capacity() {
    let mut buf = vec![0; 11].writer();
    let num = buf.write(&b"hello world"[..]).unwrap();
}

#[test]
fn test_writer_with_excess_capacity() {
    let mut buf = vec![0; 20].writer();
    let num = buf.write(&b"hello world"[..]).unwrap();
}

#[test]
fn test_writer_with_exact_capacity() {
    let mut buf = vec![0; 11].writer();
    let num = buf.write(&b"hello world"[..]).unwrap();
}

#[test]
fn test_writer_with_edge_case() {
    let mut buf = Vec::<u8>::new().writer();
    buf.put_slice(&b"hello world"[..]);
}

