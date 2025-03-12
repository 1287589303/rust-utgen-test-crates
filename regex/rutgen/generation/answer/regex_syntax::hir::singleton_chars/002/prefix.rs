// Answer 0

#[test]
fn test_singleton_chars_invalid_utf8_character() {
    struct Literal(Vec<u8>);

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal(vec![0, 159, 146, 150])), // Invalid UTF-8 sequence
            props: Properties::default(),
        },
    ];
    singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_valid_bytes_but_invalid_character() {
    struct Literal(Vec<u8>);

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal(vec![0xC0, 0x80])), // Valid bytes but decode to unassignable character
            props: Properties::default(),
        },
    ];
    singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_empty_byte_sequence() {
    struct Literal(Vec<u8>);

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal(vec![])), // Empty byte sequence
            props: Properties::default(),
        },
    ];
    singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_invalid_starting_byte() {
    struct Literal(Vec<u8>);

    let hirs = vec![
        Hir {
            kind: HirKind::Literal(Literal(vec![0xFF])), // Invalid starting byte for UTF-8
            props: Properties::default(),
        },
    ];
    singleton_chars(&hirs);
}

