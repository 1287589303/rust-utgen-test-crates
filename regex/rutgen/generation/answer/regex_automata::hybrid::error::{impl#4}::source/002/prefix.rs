// Answer 0

#[test]
fn test_source_cache_error() {
    let cache_error = CacheError(());
    let start_error = StartError::Cache { err: cache_error };
    let _result = start_error.source();
}

#[test]
fn test_source_cache_error_with_different_instance() {
    let cache_error = CacheError(());
    let start_error = StartError::Cache { err: cache_error.clone() };
    let _result = start_error.source();
}

