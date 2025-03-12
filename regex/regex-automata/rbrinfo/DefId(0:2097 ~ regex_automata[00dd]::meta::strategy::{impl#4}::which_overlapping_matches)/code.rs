fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        if let Some(e) = self.dfa.get(input) {
            trace!(
                "using full DFA for overlapping search at {:?}",
                input.get_span()
            );
            let _err = match e.try_which_overlapping_matches(input, patset) {
                Ok(()) => return,
                Err(err) => err,
            };
            trace!("fast overlapping search failed: {}", _err);
        } else if let Some(e) = self.hybrid.get(input) {
            trace!(
                "using lazy DFA for overlapping search at {:?}",
                input.get_span()
            );
            let _err = match e.try_which_overlapping_matches(
                &mut cache.hybrid,
                input,
                patset,
            ) {
                Ok(()) => {
                    return;
                }
                Err(err) => err,
            };
            trace!("fast overlapping search failed: {}", _err);
        }
        trace!(
            "using PikeVM for overlapping search at {:?}",
            input.get_span()
        );
        let e = self.pikevm.get();
        e.which_overlapping_matches(&mut cache.pikevm, input, patset)
    }