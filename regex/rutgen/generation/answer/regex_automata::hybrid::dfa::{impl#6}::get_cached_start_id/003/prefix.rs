// Answer 0

#[test]
fn test_get_cached_start_id_pattern_unsupported() {
    struct DummyDFA {
        config: Config,
    }

    impl DummyDFA {
        fn get_config(&self) -> &Config {
            &self.config
        }
    }

    struct DummyCache {
        starts: Vec<LazyStateID>,
    }

    struct DummyLazyRef<'i, 'c> {
        dfa: &'i DummyDFA,
        cache: &'c DummyCache,
    }

    impl<'i, 'c> DummyLazyRef<'i, 'c> {
        fn get_cached_start_id(
            &self,
            anchored: Anchored,
            start: Start,
        ) -> Result<LazyStateID, StartError> {
            let start_index = start.as_usize();
            let index = match anchored {
                Anchored::No => start_index,
                Anchored::Yes => Start::len() + start_index,
                Anchored::Pattern(pid) => {
                    if !self.dfa.get_config().get_starts_for_each_pattern() {
                        return Err(StartError::unsupported_anchored(anchored));
                    }
                    0 // To avoid panic.
                }
            };
            Ok(self.cache.starts[index])
        }
    }

    let config = Config::default().starts_for_each_pattern(false);
    let dfa = DummyDFA { config };
    let cache = DummyCache {
        starts: vec![LazyStateID(0); 10],
    };
    let lazy_ref = DummyLazyRef { dfa: &dfa, cache: &cache };

    lazy_ref.get_cached_start_id(Anchored::Pattern(PatternID(0)), Start::WordByte).unwrap_err();
}

