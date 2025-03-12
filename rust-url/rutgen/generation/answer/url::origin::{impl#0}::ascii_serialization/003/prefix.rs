// Answer 0

#[test]
fn test_ascii_serialization_opaque() {
    let origin = Origin::Opaque(OpaqueOrigin(0usize));
    let result = origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_opaque_large_value() {
    let origin = Origin::Opaque(OpaqueOrigin(usize::MAX));
    let result = origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_opaque_non_zero_value() {
    let origin = Origin::Opaque(OpaqueOrigin(1usize));
    let result = origin.ascii_serialization();
}

