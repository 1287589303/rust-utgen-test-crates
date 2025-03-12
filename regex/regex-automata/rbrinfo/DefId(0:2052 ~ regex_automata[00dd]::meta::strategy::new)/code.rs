pub(super) fn new(
    info: &RegexInfo,
    hirs: &[&Hir],
) -> Result<Arc<dyn Strategy>, BuildError> {
    // At this point, we're committed to a regex engine of some kind. So pull
    // out a prefilter if we can, which will feed to each of the constituent
    // regex engines.
    let pre = if info.is_always_anchored_start() {
        // PERF: I'm not sure we necessarily want to do this... We may want to
        // run a prefilter for quickly rejecting in some cases. The problem
        // is that anchored searches overlap quite a bit with the use case
        // of "run a regex on every line to extract data." In that case, the
        // regex always matches, so running a prefilter doesn't really help us
        // there. The main place where a prefilter helps in an anchored search
        // is if the anchored search is not expected to match frequently. That
        // is, the prefilter gives us a way to possibly reject a haystack very
        // quickly.
        //
        // Maybe we should do use a prefilter, but only for longer haystacks?
        // Or maybe we should only use a prefilter when we think it's "fast"?
        //
        // Interestingly, I think we currently lack the infrastructure for
        // disabling a prefilter based on haystack length. That would probably
        // need to be a new 'Input' option. (Interestingly, an 'Input' used to
        // carry a 'Prefilter' with it, but I moved away from that.)
        debug!("skipping literal extraction since regex is anchored");
        None
    } else if let Some(pre) = info.config().get_prefilter() {
        debug!(
            "skipping literal extraction since the caller provided a prefilter"
        );
        Some(pre.clone())
    } else if info.config().get_auto_prefilter() {
        let kind = info.config().get_match_kind();
        let prefixes = crate::util::prefilter::prefixes(kind, hirs);
        // If we can build a full `Strategy` from just the extracted prefixes,
        // then we can short-circuit and avoid building a regex engine at all.
        if let Some(pre) = Pre::from_prefixes(info, &prefixes) {
            debug!(
                "found that the regex can be broken down to a literal \
                 search, avoiding the regex engine entirely",
            );
            return Ok(pre);
        }
        // This now attempts another short-circuit of the regex engine: if we
        // have a huge alternation of just plain literals, then we can just use
        // Aho-Corasick for that and avoid the regex engine entirely.
        //
        // You might think this case would just be handled by
        // `Pre::from_prefixes`, but that technique relies on heuristic literal
        // extraction from the corresponding `Hir`. That works, but part of
        // heuristics limit the size and number of literals returned. This case
        // will specifically handle patterns with very large alternations.
        //
        // One wonders if we should just roll this our heuristic literal
        // extraction, and then I think this case could disappear entirely.
        if let Some(pre) = Pre::from_alternation_literals(info, hirs) {
            debug!(
                "found plain alternation of literals, \
                 avoiding regex engine entirely and using Aho-Corasick"
            );
            return Ok(pre);
        }
        prefixes.literals().and_then(|strings| {
            debug!(
                "creating prefilter from {} literals: {:?}",
                strings.len(),
                strings,
            );
            Prefilter::new(kind, strings)
        })
    } else {
        debug!("skipping literal extraction since prefilters were disabled");
        None
    };
    let mut core = Core::new(info.clone(), pre.clone(), hirs)?;
    // Now that we have our core regex engines built, there are a few cases
    // where we can do a little bit better than just a normal "search forward
    // and maybe use a prefilter when in a start state." However, these cases
    // may not always work or otherwise build on top of the Core searcher.
    // For example, the reverse anchored optimization seems like it might
    // always work, but only the DFAs support reverse searching and the DFAs
    // might give up or quit for reasons. If we had, e.g., a PikeVM that
    // supported reverse searching, then we could avoid building a full Core
    // engine for this case.
    core = match ReverseAnchored::new(core) {
        Err(core) => core,
        Ok(ra) => {
            debug!("using reverse anchored strategy");
            return Ok(Arc::new(ra));
        }
    };
    core = match ReverseSuffix::new(core, hirs) {
        Err(core) => core,
        Ok(rs) => {
            debug!("using reverse suffix strategy");
            return Ok(Arc::new(rs));
        }
    };
    core = match ReverseInner::new(core, hirs) {
        Err(core) => core,
        Ok(ri) => {
            debug!("using reverse inner strategy");
            return Ok(Arc::new(ri));
        }
    };
    debug!("using core strategy");
    Ok(Arc::new(core))
}