// Answer 0

#[test]
fn test_try_into_mut_from_static_non_unique() {
    let data: &'static [u8] = b"hello";
    let bytes = Bytes::from_static(data);
    // Simulate non-uniqueness (e.g., by cloning or sharing)
    let _shared = bytes.clone(); // Create a clone to share
    let result = bytes.try_into_mut();
}

#[test]
fn test_try_into_mut_from_owner_non_unique() {
    struct Owner {
        data: Vec<u8>,
    }

    let owner = Owner { data: b"hello".to_vec() };
    let bytes = Bytes::from_owner(owner);
    // Simulate non-uniqueness (e.g., by cloning or sharing)
    let _shared = bytes.clone(); // Create a clone to share
    let result = bytes.try_into_mut();
}

