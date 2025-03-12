// Answer 0

#[test]
fn test_empty_array() {
    let bytes_ref = BytesRef(&[]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_single_zero() {
    let bytes_ref = BytesRef(&[0]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_single_max() {
    let bytes_ref = BytesRef(&[255]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_duplicate_values() {
    let bytes_ref = BytesRef(&[1, 1, 2, 2, 3, 3]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_all_bytes() {
    let bytes_ref = BytesRef(&(0..=255).collect::<Vec<u8>>());
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_max_length_array() {
    let max_length = 1024; // Example maximum length, adjust per system capability
    let bytes_ref = BytesRef(&vec![0; max_length]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

