// Answer 0

#[test]
fn test_bytes_with_single_capture_reference() {
    let mut dst = Vec::new();
    let replacement = b"Hello $name!";
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"Alice");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "name" {
            Some(0)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_multiple_capture_references() {
    let mut dst = Vec::new();
    let replacement = b"$user is logged in as $role.";
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        match index {
            0 => dst.extend_from_slice(b"john_doe"),
            1 => dst.extend_from_slice(b"admin"),
            _ => {}
        }
    };
    let mut name_to_index = |name: &str| {
        match name {
            "user" => Some(0),
            "role" => Some(1),
            _ => None,
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_no_capture_references() {
    let mut dst = Vec::new();
    let replacement = b"No captures here!";
    let append = |_: usize, _: &mut Vec<u8>| {};
    let name_to_index = |_name: &str| None;

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_edge_case_capture_reference() {
    let mut dst = Vec::new();
    let replacement = b"Value: ${value}";
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"42");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "value" {
            Some(0)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

