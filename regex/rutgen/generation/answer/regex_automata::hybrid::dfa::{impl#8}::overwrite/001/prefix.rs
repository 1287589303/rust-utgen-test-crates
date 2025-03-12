// Answer 0

#[test]
fn test_overwrite_all_options_set() {
    let default_config = Config::new()
        .match_kind(MatchKind::All)
        .prefilter(Some(Prefilter {
            pre: Arc::new(DummyPrefilter {}),
            is_fast: true,
            max_needle_len: 10,
        }))
        .starts_for_each_pattern(true)
        .byte_classes(true)
        .unicode_word_boundary(true)
        .quit(1, true)
        .specialize_start_states(true)
        .cache_capacity(1024)
        .skip_cache_capacity_check(true)
        .minimum_cache_clear_count(Some(0))
        .minimum_bytes_per_state(Some(0));
        
    let new_config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .prefilter(Some(Prefilter {
            pre: Arc::new(DummyPrefilter {}),
            is_fast: false,
            max_needle_len: 5,
        }))
        .starts_for_each_pattern(false)
        .byte_classes(false)
        .unicode_word_boundary(false)
        .quit(2, false)
        .specialize_start_states(false)
        .cache_capacity(512)
        .skip_cache_capacity_check(false)
        .minimum_cache_clear_count(Some(1))
        .minimum_bytes_per_state(Some(1));

    let result = default_config.overwrite(new_config);
}

#[test]
fn test_overwrite_all_options_unset() {
    let default_config = Config::new();
    
    let new_config = Config::new();

    let result = default_config.overwrite(new_config);
}

#[test]
fn test_overwrite_some_options_set() {
    let default_config = Config::new()
        .match_kind(MatchKind::All)
        .prefilter(None)
        .starts_for_each_pattern(true)
        .byte_classes(false)
        .unicode_word_boundary(false);
        
    let new_config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .prefilter(Some(Prefilter {
            pre: Arc::new(DummyPrefilter {}),
            is_fast: true,
            max_needle_len: 3,
        }))
        .starts_for_each_pattern(false)
        .cache_capacity(256);

    let result = default_config.overwrite(new_config);
} 

struct DummyPrefilter;

impl PrefilterI for DummyPrefilter {}

