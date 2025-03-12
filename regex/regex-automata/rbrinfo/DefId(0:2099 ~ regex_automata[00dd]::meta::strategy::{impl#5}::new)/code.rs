fn new(core: Core) -> Result<ReverseAnchored, Core> {
        if !core.info.is_always_anchored_end() {
            debug!(
                "skipping reverse anchored optimization because \
				 the regex is not always anchored at the end"
            );
            return Err(core);
        }
        // Note that the caller can still request an anchored search even when
        // the regex isn't anchored at the start. We detect that case in the
        // search routines below and just fallback to the core engine. This
        // is fine because both searches are anchored. It's just a matter of
        // picking one. Falling back to the core engine is a little simpler,
        // since if we used the reverse anchored approach, we'd have to add an
        // extra check to ensure the match reported starts at the place where
        // the caller requested the search to start.
        if core.info.is_always_anchored_start() {
            debug!(
                "skipping reverse anchored optimization because \
				 the regex is also anchored at the start"
            );
            return Err(core);
        }
        // Only DFAs can do reverse searches (currently), so we need one of
        // them in order to do this optimization. It's possible (although
        // pretty unlikely) that we have neither and need to give up.
        if !core.hybrid.is_some() && !core.dfa.is_some() {
            debug!(
                "skipping reverse anchored optimization because \
				 we don't have a lazy DFA or a full DFA"
            );
            return Err(core);
        }
        Ok(ReverseAnchored { core })
    }