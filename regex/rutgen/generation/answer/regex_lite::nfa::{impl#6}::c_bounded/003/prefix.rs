// Answer 0

#[test]
fn test_c_bounded_successful_path() {
    let config = Config { nest_limit: 5 };
    let pattern = String::from("a{2,5}");
    let compiler = Compiler::new(config.clone(), pattern.clone());
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 2,
            max: 5,
            greedy: true,
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let min = 2;
    let max = 5;
    let greedy = true;

    compiler.c_bounded(&hir, greedy, min, max).unwrap();
}

#[test]
#[should_panic]
fn test_c_bounded_empty_error() {
    let config = Config { nest_limit: 5 };
    let pattern = String::from("a{2,5}");
    let compiler = Compiler::new(config.clone(), pattern.clone());
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 2,
            max: 5,
            greedy: true,
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let min = 2;
    let max = 5;
    let greedy = false;

    // This should panic due to self.add_empty()? returning an Err/None
    compiler.c_bounded(&hir, greedy, min, max).unwrap();
}

