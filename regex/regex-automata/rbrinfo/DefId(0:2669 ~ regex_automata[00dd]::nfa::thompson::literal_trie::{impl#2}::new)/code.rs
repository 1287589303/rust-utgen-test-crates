fn new(state: &'a State) -> Frame<'a> {
        let mut chunks = state.chunks();
        // every state has at least 1 chunk
        let chunk = chunks.next().unwrap();
        let transitions = chunk.iter();
        Frame { chunks, transitions, union: vec![], sparse: vec![] }
    }