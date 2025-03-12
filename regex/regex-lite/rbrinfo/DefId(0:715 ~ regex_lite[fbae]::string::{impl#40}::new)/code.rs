pub fn new(pattern: &str) -> RegexBuilder {
        RegexBuilder {
            pattern: pattern.to_string(),
            hir_config: hir::Config::default(),
            nfa_config: nfa::Config::default(),
        }
    }