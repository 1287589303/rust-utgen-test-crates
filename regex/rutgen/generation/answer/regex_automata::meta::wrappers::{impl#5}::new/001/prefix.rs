// Answer 0

#[test]
fn test_new_with_backtrack_enabled() {
    struct DummyBoundedBacktracker {
        engine: Option<backtrack::BoundedBacktracker>,
    }

    let builder = DummyBoundedBacktracker {
        engine: Some(backtrack::BoundedBacktracker { /* initialize fields */ }),
    };
    let _cache = BoundedBacktrackerCache::new(&builder);
}

#[test]
#[cfg(not(feature = "nfa-backtrack"))]
fn test_new_with_backtrack_disabled() {
    struct DummyBoundedBacktracker {
        engine: Option<backtrack::BoundedBacktracker>,
    }

    let builder = DummyBoundedBacktracker {
        engine: None,
    };
    let _cache = BoundedBacktrackerCache::new(&builder);
}

