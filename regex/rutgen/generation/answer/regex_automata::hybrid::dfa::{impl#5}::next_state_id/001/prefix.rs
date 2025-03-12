// Answer 0

#[test]
fn test_next_state_id_err_lazy_state_id() {
    struct TestDFA {
        cache: Cache,
    }

    impl TestDFA {
        fn new() -> Self {
            let mut cache = Cache {
                trans: vec![LazyStateID(0); LazyStateID::MAX + 2],
                clear_count: LazyStateID::MAX as usize + 1,
            };
            Self { cache }
        }

        fn next_state_id(&mut self) -> Result<LazyStateID, CacheError> {
            let sid = match LazyStateID::new(self.cache.trans.len()) {
                Ok(sid) => sid,
                Err(_) => {
                    self.try_clear_cache()?;
                    LazyStateID::new(self.cache.trans.len()).unwrap()
                }
            };
            Ok(sid)
        }

        fn try_clear_cache(&mut self) -> Result<(), CacheError> {
            Err(CacheError(()))
        }
    }

    let mut test_dfa = TestDFA::new();
    let result = test_dfa.next_state_id();
}

#[test]
fn test_next_state_id_no_clear_cache() {
    struct TestDFA {
        cache: Cache,
    }

    impl TestDFA {
        fn new() -> Self {
            let mut cache = Cache {
                trans: vec![LazyStateID(0); LazyStateID::MAX + 2],
                clear_count: LazyStateID::MAX as usize + 1,
            };
            Self { cache }
        }

        fn next_state_id(&mut self) -> Result<LazyStateID, CacheError> {
            let sid = match LazyStateID::new(self.cache.trans.len()) {
                Ok(sid) => sid,
                Err(_) => {
                    self.try_clear_cache()?;
                    LazyStateID::new(self.cache.trans.len()).unwrap()
                }
            };
            Ok(sid)
        }

        fn try_clear_cache(&mut self) -> Result<(), CacheError> {
            // Simulating no error
            Ok(())
        }
    }

    let mut test_dfa = TestDFA::new();
    let result = test_dfa.next_state_id();
}

