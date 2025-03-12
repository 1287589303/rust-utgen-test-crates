// Answer 0

#[test]
fn test_new_hybrid_cache_with_some() {
    struct Hybrid {
        regex: Option<hybrid::regex::Regex>,
    }
    
    impl Hybrid {
        fn new_some() -> Self {
            Hybrid {
                regex: Some(hybrid::regex::Regex::new("test").unwrap()),
            }
        }
    }
    
    let builder = Hybrid::new_some();
    let _cache = HybridCache::new(&builder);
}

#[test]
fn test_new_hybrid_cache_with_none() {
    struct Hybrid {
        regex: Option<hybrid::regex::Regex>,
    }
    
    impl Hybrid {
        fn new_none() -> Self {
            Hybrid {
                regex: None,
            }
        }
    }
    
    let builder = Hybrid::new_none();
    let _cache = HybridCache::new(&builder);
}

