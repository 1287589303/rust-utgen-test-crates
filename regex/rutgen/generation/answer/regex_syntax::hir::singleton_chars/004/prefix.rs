// Answer 0

#[test]
fn test_singleton_chars_valid_single_byte_literals() {
    let hirs = vec![
        Hir { 
            kind: HirKind::Literal(Literal(vec![b'a'])),
            props: Properties {},
        },
        Hir { 
            kind: HirKind::Literal(Literal(vec![b'b'])),
            props: Properties {},
        },
    ];
    let _ = singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_valid_multi_byte_literals() {
    let hirs = vec![
        Hir { 
            kind: HirKind::Literal(Literal(vec![0b11000010, 0b10111100])), // 'À' in UTF-8
            props: Properties {},
        },
        Hir { 
            kind: HirKind::Literal(Literal(vec![0b11000010, 0b10111101])), // 'Á' in UTF-8
            props: Properties {},
        },
    ];
    let _ = singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_utf8_decode_err() {
    let hirs = vec![
        Hir { 
            kind: HirKind::Literal(Literal(vec![0b11000000])), // Invalid UTF-8 sequence
            props: Properties {},
        },
    ];
    let _ = singleton_chars(&hirs);
}

#[test]
fn test_singleton_chars_empty_vector() {
    let hirs: Vec<Hir> = vec![];
    let _ = singleton_chars(&hirs);
} 

#[test]
fn test_singleton_chars_invalid_length() {
    let hirs = vec![
        Hir { 
            kind: HirKind::Literal(Literal(vec![b'a'])),
            props: Properties {},
        },
        Hir { 
            kind: HirKind::Literal(Literal(vec![0b11000010, 0b10111100, b'x'])), // 'À' but with an extra byte
            props: Properties {},
        },
    ];  
    let _ = singleton_chars(&hirs);
}

