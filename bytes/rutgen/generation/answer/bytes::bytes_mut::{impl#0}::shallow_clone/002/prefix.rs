// Answer 0

#[test]
fn test_shallow_clone_vec_kind_with_min_capacity() {
    let mut bytes_mut = unsafe {
        let mut buf = BytesMut::with_capacity(2);
        bytes_mut.set_len(2);
        bytes_mut
    };
    let _ = unsafe { bytes_mut.shallow_clone() };
}

#[test]
fn test_shallow_clone_vec_kind_with_more_than_min_capacity() {
    let mut bytes_mut = unsafe {
        let mut buf = BytesMut::with_capacity(10);
        bytes_mut.set_len(10);
        bytes_mut
    };
    let _ = unsafe { bytes_mut.shallow_clone() };
}

#[test]
fn test_shallow_clone_vec_kind_with_exact_capacity() {
    let mut bytes_mut = unsafe {
        let mut buf = BytesMut::with_capacity(17);
        bytes_mut.set_len(17);
        bytes_mut
    };
    let _ = unsafe { bytes_mut.shallow_clone() };
}

