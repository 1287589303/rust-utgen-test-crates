// Answer 0

#[test]
fn test_concat_with_adjacent_literals() {
    let lit1 = Hir::literal([b'a']);
    let lit2 = Hir::literal([b'b']);
    let lit3 = Hir::literal([b'c']);
    let lit4 = Hir::literal([b'd']);
    let lit5 = Hir::literal([b'e']);
    
    let hir = Hir::concat(vec![
        Hir::concat(vec![lit1, lit2]),
        Hir::concat(vec![lit3, lit4, lit5]),
    ]);
}

#[test]
fn test_concat_with_flattens_multiple_concats() {
    let lit1 = Hir::literal([b'x']);
    let lit2 = Hir::literal([b'y']);
    let lit3 = Hir::literal([b'z']);
    
    let hir = Hir::concat(vec![
        Hir::concat(vec![lit1, lit2]),
        Hir::concat(vec![lit3]),
    ]);
}

#[test]
fn test_concat_with_single_concat_result() {
    let lit1 = Hir::literal([b'1']);
    let lit2 = Hir::literal([b'2']);
    
    let hir = Hir::concat(vec![
        Hir::literal([b'0']),
        Hir::concat(vec![lit1, lit2]),
    ]);
}

#[test]
fn test_concat_with_non_adjacent_literals() {
    let lit1 = Hir::literal([b'h']);
    let lit2 = Hir::literal([b'i']);
    
    let hir = Hir::concat(vec![
        Hir::concat(vec![lit1]),
        Hir::literal([b'j']),
        Hir::concat(vec![lit2]),
    ]);
}

