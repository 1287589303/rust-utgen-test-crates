// Answer 0

#[test]
fn test_c_at_least_n_greater_than_zero_match_empty_false_patch_fail() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a+";
    let hir = Hir::parse(config.clone(), pattern).expect("Failed to parse Hir");
    let compiler = Compiler::new(config, pattern.to_string());

    let greedy = true;
    let n = 1;

    let splits_result = compiler.add(State::Splits { targets: vec![1, 2], reverse: !greedy });
    if let Ok(splits) = splits_result {
        let compiled_result = compiler.c(&hir);
        if let Ok(compiled) = compiled_result {
            let patch_result_start = compiler.patch(splits, compiled.start);
            if patch_result_start.is_ok() {
                let patch_result_end = compiler.patch(compiled.end, splits);
                // We expect this patch to fail
                assert!(patch_result_end.is_err());
            }
        }
    }
}

#[test]
fn test_c_at_least_n_greater_than_zero_match_empty_false_patch_success() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "b*";
    let hir = Hir::parse(config.clone(), pattern).expect("Failed to parse Hir");
    let compiler = Compiler::new(config, pattern.to_string());

    let greedy = false;
    let n = 2;

    let splits_result = compiler.add(State::Splits { targets: vec![3, 4], reverse: !greedy });
    if let Ok(splits) = splits_result {
        let compiled_result = compiler.c(&hir);
        if let Ok(compiled) = compiled_result {
            let patch_result_start = compiler.patch(splits, compiled.start);
            if patch_result_start.is_ok() {
                let patch_result_end = compiler.patch(compiled.end, splits);
                // We expect this patch to be Ok
                assert!(patch_result_end.is_ok());
            }
        }
    }
}

