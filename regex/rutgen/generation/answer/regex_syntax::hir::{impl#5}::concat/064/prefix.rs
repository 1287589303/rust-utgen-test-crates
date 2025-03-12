// Answer 0

#[test]
fn test_concat_with_multiple_literals() {
    let hir1 = Hir::literal([b'a']);
    let hir2 = Hir::literal([b'b']);
    let hir3 = Hir::literal([b'c']);
    let hir4 = Hir::literal([b'd']);
    
    let hir = Hir::concat(vec![
        hir1,
        Hir::concat(vec![
            hir2,
            hir3,
        ]),
        hir4,
    ]);
}

#[test]
fn test_concat_with_empty_and_literls() {
    let empty_hir = Hir::empty();
    let hir1 = Hir::literal([b'x']);
    let hir2 = Hir::literal([b'y']);
    
    let hir = Hir::concat(vec![
        empty_hir,
        hir1,
        Hir::concat(vec![
            hir2,
            Hir::literal([b'z']),
        ]),
    ]);
}

#[test]
fn test_concat_with_consecutive_literls() {
    let hir1 = Hir::literal([b'1']);
    let hir2 = Hir::literal([b'2']);
    let hir3 = Hir::literal([b'3']);
    
    let hir = Hir::concat(vec![
        Hir::concat(vec![
            hir1,
            hir2,
        ]),
        hir3,
    ]);
}

