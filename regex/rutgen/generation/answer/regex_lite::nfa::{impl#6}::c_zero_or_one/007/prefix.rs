// Answer 0

#[test]
fn test_c_zero_or_one_greedy() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a*"));
    
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 0,
            max: None,
            greedy: true,
            sub: Box::new(Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None })
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let _ = compiler.c_zero_or_one(&hir, true);
}

#[test]
fn test_c_zero_or_one_non_greedy() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a*"));
    
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 0,
            max: None,
            greedy: false,
            sub: Box::new(Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None })
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let _ = compiler.c_zero_or_one(&hir, false);
}

#[test]
fn test_c_zero_or_one_empty() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("")); // empty pattern

    let hir = Hir { 
        kind: HirKind::Empty, 
        is_start_anchored: false, 
        is_match_empty: true, 
        static_explicit_captures_len: None 
    };
    
    let _ = compiler.c_zero_or_one(&hir, true);
}

#[test]
fn test_c_zero_or_one_char() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a"));

    let hir = Hir { 
        kind: HirKind::Char('a'), 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };
    
    let _ = compiler.c_zero_or_one(&hir, false);
}

#[test]
fn test_c_zero_or_one_class() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("[a-z]"));

    let class = hir::Class { ranges: vec![('a', 'z')] };
    let hir = Hir { 
        kind: HirKind::Class(Box::new(class)), 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };

    let _ = compiler.c_zero_or_one(&hir, true);
}

