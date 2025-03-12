fn is_match_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> bool {
        if let Some(ref e) = self.onepass.get(input) {
            trace!(
                "using OnePass for is-match search at {:?}",
                input.get_span()
            );
            e.search_slots(&mut cache.onepass, input, &mut []).is_some()
        } else if let Some(ref e) = self.backtrack.get(input) {
            trace!(
                "using BoundedBacktracker for is-match search at {:?}",
                input.get_span()
            );
            e.is_match(&mut cache.backtrack, input)
        } else {
            trace!(
                "using PikeVM for is-match search at {:?}",
                input.get_span()
            );
            let e = self.pikevm.get();
            e.is_match(&mut cache.pikevm, input)
        }
    }