// Answer 0

#[test]
fn test_is_accelerated_literal() {
    let strat = Arc::new(MyStrategy { accelerated: true });
    let info = RegexInfo::new();
    let regex = Regex {
        imp: Arc::new(RegexI { strat, info }),
        pool: CachePool::new(),
    };
    regex.is_accelerated();
}

#[test]
fn test_is_accelerated_complex_pattern() {
    let strat = Arc::new(MyStrategy { accelerated: true });
    let info = RegexInfo::new();
    let regex = Regex {
        imp: Arc::new(RegexI { strat, info }),
        pool: CachePool::new(),
    };
    regex.is_accelerated();
}

#[test]
fn test_is_not_accelerated_no_literals() {
    let strat = Arc::new(MyStrategy { accelerated: false });
    let info = RegexInfo::new();
    let regex = Regex {
        imp: Arc::new(RegexI { strat, info }),
        pool: CachePool::new(),
    };
    regex.is_accelerated();
}

#[test]
fn test_is_not_accelerated_escape_sequences() {
    let strat = Arc::new(MyStrategy { accelerated: false });
    let info = RegexInfo::new();
    let regex = Regex {
        imp: Arc::new(RegexI { strat, info }),
        pool: CachePool::new(),
    };
    regex.is_accelerated();
}

#[test]
fn test_is_accelerated_numbers() {
    let strat = Arc::new(MyStrategy { accelerated: true });
    let info = RegexInfo::new();
    let regex = Regex {
        imp: Arc::new(RegexI { strat, info }),
        pool: CachePool::new(),
    };
    regex.is_accelerated();
}

