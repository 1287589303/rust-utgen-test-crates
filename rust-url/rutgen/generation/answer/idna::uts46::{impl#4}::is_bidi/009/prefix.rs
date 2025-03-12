// Answer 0

#[test]
fn test_is_bidi_all_below_hebrew() {
    let uts46 = Uts46::new();
    let buffer = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let result = uts46.is_bidi(&buffer);
}

#[test]
fn test_is_bidi_range_0900_fb1c() {
    let uts46 = Uts46::new();
    let buffer = vec!['\u{0900}', '\u{0901}', '\u{0902}'];
    let result = uts46.is_bidi(&buffer);
}

#[test]
fn test_is_bidi_range_1F000_3FFFF() {
    let uts46 = Uts46::new();
    let buffer = vec!['\u{1F000}', '\u{1F001}', '\u{1F002}'];
    let result = uts46.is_bidi(&buffer);
}

#[test]
fn test_is_bidi_range_FF00_107FF() {
    let uts46 = Uts46::new();
    let buffer = vec!['\u{FF00}', '\u{FF01}', '\u{FF02}'];
    let result = uts46.is_bidi(&buffer);
}

#[test]
fn test_is_bidi_range_11000_1E7FF() {
    let uts46 = Uts46::new();
    let buffer = vec!['\u{11000}', '\u{11001}', '\u{11002}'];
    let result = uts46.is_bidi(&buffer);
}

