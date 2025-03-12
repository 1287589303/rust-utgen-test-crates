pub(super) fn new(r: &impl Remappable) -> Remapper {
        let idxmap = IndexMapper { stride2: r.stride2() };
        let map = (0..r.state_len()).map(|i| idxmap.to_state_id(i)).collect();
        Remapper { map, idxmap }
    }