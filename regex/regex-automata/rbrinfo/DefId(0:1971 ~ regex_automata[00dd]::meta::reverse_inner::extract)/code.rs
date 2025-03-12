pub(crate) fn extract(hirs: &[&Hir]) -> Option<(Hir, Prefilter)> {
    if hirs.len() != 1 {
        debug!(
            "skipping reverse inner optimization since it only \
		 	 supports 1 pattern, {} were given",
            hirs.len(),
        );
        return None;
    }
    let mut concat = match top_concat(hirs[0]) {
        Some(concat) => concat,
        None => {
            debug!(
                "skipping reverse inner optimization because a top-level \
		 	     concatenation could not found",
            );
            return None;
        }
    };
    // We skip the first HIR because if it did have a prefix prefilter in it,
    // we probably wouldn't be here looking for an inner prefilter.
    for i in 1..concat.len() {
        let hir = &concat[i];
        let pre = match prefilter(hir) {
            None => continue,
            Some(pre) => pre,
        };
        // Even if we got a prefilter, if it isn't consider "fast," then we
        // probably don't want to bother with it. Namely, since the reverse
        // inner optimization requires some overhead, it likely only makes
        // sense if the prefilter scan itself is (believed) to be much faster
        // than the regex engine.
        if !pre.is_fast() {
            debug!(
                "skipping extracted inner prefilter because \
				 it probably isn't fast"
            );
            continue;
        }
        let concat_suffix = Hir::concat(concat.split_off(i));
        let concat_prefix = Hir::concat(concat);
        // Look for a prefilter again. Why? Because above we only looked for
        // a prefilter on the individual 'hir', but we might be able to find
        // something better and more discriminatory by looking at the entire
        // suffix. We don't do this above to avoid making this loop worst case
        // quadratic in the length of 'concat'.
        let pre2 = match prefilter(&concat_suffix) {
            None => pre,
            Some(pre2) => {
                if pre2.is_fast() {
                    pre2
                } else {
                    pre
                }
            }
        };
        return Some((concat_prefix, pre2));
    }
    debug!(
        "skipping reverse inner optimization because a top-level \
	     sub-expression with a fast prefilter could not be found"
    );
    None
}