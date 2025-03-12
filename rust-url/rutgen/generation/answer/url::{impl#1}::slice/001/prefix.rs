// Answer 0

#[test]
fn test_slice_range_0_to_0() {
    let url = Url::parse("http://example.com").unwrap();
    url.slice(0..0);
}

#[test]
fn test_slice_range_0_to_length() {
    let url = Url::parse("http://example.com").unwrap();
    let length = url.slice(0..0).len(); // hoping length matches serialization
    url.slice(0..length as u32);
}

#[test]
fn test_slice_range_between() {
    let url = Url::parse("http://example.com").unwrap();
    let length = url.slice(0..0).len();
    url.slice(1..length as u32);
}

#[test]
fn test_slice_range_length_minus_1_to_length() {
    let url = Url::parse("http://example.com").unwrap();
    let length = url.slice(0..0).len();
    url.slice((length - 1) as u32..length as u32);
}

#[test]
fn test_slice_full_range() {
    let url = Url::parse("http://example.com").unwrap();
    let length = url.slice(0..0).len();
    url.slice(0..length as u32);
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_start() {
    let url = Url::parse("http://example.com").unwrap();
    let length = url.slice(0..0).len();
    url.slice(length as u32..length as u32 + 1);
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_end() {
    let url = Url::parse("http://example.com").unwrap();
    url.slice(0..5); // assuming serialization < 5
}

