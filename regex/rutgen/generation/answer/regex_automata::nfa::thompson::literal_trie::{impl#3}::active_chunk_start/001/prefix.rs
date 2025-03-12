// Answer 0

#[test]
fn test_active_chunk_start_empty_chunks() {
    let state = State {
        chunks: Vec::new(),
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

#[test]
fn test_active_chunk_start_single_chunk() {
    let state = State {
        chunks: vec![(0, 10)],
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

#[test]
fn test_active_chunk_start_multiple_chunks() {
    let state = State {
        chunks: vec![(0, 5), (5, 15), (15, 20)],
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

#[test]
fn test_active_chunk_start_single_entry_zero_end() {
    let state = State {
        chunks: vec![(0, 0)],
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

#[test]
fn test_active_chunk_start_multiple_chunks_with_zero_end() {
    let state = State {
        chunks: vec![(0, 5), (5, 0)],
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

#[test]
fn test_active_chunk_start_large_end_value() {
    let state = State {
        chunks: vec![(0, usize::MAX as usize), (1, 20)],
        ..Default::default()
    };
    let _ = state.active_chunk_start();
}

