fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseSuffix, Core> {
        if !core.info.config().get_auto_prefilter() {
            debug!(
                "skipping reverse suffix optimization because \
                 automatic prefilters are disabled"
            );
            return Err(core);
        }
        // Like the reverse inner optimization, we don't do this for regexes
        // that are always anchored. It could lead to scanning too much, but
        // could say "no match" much more quickly than running the regex
        // engine if the initial literal scan doesn't match. With that said,
        // the reverse suffix optimization has lower overhead, since it only
        // requires a reverse scan after a literal match to confirm or reject
        // the match. (Although, in the case of confirmation, it then needs to
        // do another forward scan to find the end position.)
        //
        // Note that the caller can still request an anchored search even
        // when the regex isn't anchored. We detect that case in the search
        // routines below and just fallback to the core engine. Currently this
        // optimization assumes all searches are unanchored, so if we do want
        // to enable this optimization for anchored searches, it will need a
        // little work to support it.
        if core.info.is_always_anchored_start() {
            debug!(
                "skipping reverse suffix optimization because \
				 the regex is always anchored at the start",
            );
            return Err(core);
        }
        // Only DFAs can do reverse searches (currently), so we need one of
        // them in order to do this optimization. It's possible (although
        // pretty unlikely) that we have neither and need to give up.
        if !core.hybrid.is_some() && !core.dfa.is_some() {
            debug!(
                "skipping reverse suffix optimization because \
				 we don't have a lazy DFA or a full DFA"
            );
            return Err(core);
        }
        if core.pre.as_ref().map_or(false, |p| p.is_fast()) {
            debug!(
                "skipping reverse suffix optimization because \
				 we already have a prefilter that we think is fast"
            );
            return Err(core);
        }
        let kind = core.info.config().get_match_kind();
        let suffixes = crate::util::prefilter::suffixes(kind, hirs);
        let lcs = match suffixes.longest_common_suffix() {
            None => {
                debug!(
                    "skipping reverse suffix optimization because \
                     a longest common suffix could not be found",
                );
                return Err(core);
            }
            Some(lcs) if lcs.is_empty() => {
                debug!(
                    "skipping reverse suffix optimization because \
                     the longest common suffix is the empty string",
                );
                return Err(core);
            }
            Some(lcs) => lcs,
        };
        let pre = match Prefilter::new(kind, &[lcs]) {
            Some(pre) => pre,
            None => {
                debug!(
                    "skipping reverse suffix optimization because \
                     a prefilter could not be constructed from the \
                     longest common suffix",
                );
                return Err(core);
            }
        };
        if !pre.is_fast() {
            debug!(
                "skipping reverse suffix optimization because \
				 while we have a suffix prefilter, it is not \
				 believed to be 'fast'"
            );
            return Err(core);
        }
        Ok(ReverseSuffix { core, pre })
    }