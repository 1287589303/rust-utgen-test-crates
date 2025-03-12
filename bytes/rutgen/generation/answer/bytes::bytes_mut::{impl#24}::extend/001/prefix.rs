// Answer 0

#[test]
fn test_extend_empty_iterator() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let empty_iter: Vec<&u8> = Vec::new();
    bytes_mut.extend(empty_iter.into_iter());
}

#[test]
fn test_extend_single_element() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let single_element: Vec<&u8> = vec![&100];
    bytes_mut.extend(single_element.into_iter());
}

#[test]
fn test_extend_multiple_elements_within_capacity() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let multiple_elements: Vec<&u8> = vec![&100, &150, &200];
    bytes_mut.extend(multiple_elements.into_iter());
}

#[test]
fn test_extend_boundary_values() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let boundary_elements: Vec<&u8> = vec![&0, &255];
    bytes_mut.extend(boundary_elements.into_iter());
}

#[test]
fn test_extend_max_capacity() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 5,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let max_capacity_elements: Vec<&u8> = (0..5).map(|i| &(i as u8)).collect();
    bytes_mut.extend(max_capacity_elements.into_iter());
}

