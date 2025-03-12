pub(crate) fn get(
        &self,
        input: &Input<'_>,
    ) -> Option<&BoundedBacktrackerEngine> {
        let engine = self.0.as_ref()?;
        // It is difficult to make the backtracker give up early if it is
        // guaranteed to eventually wind up in a match state. This is because
        // of the greedy nature of a backtracker: it just blindly mushes
        // forward. Every other regex engine is able to give up more quickly,
        // so even if the backtracker might be able to zip through faster than
        // (say) the PikeVM, we prefer the theoretical benefit that some other
        // engine might be able to scan much less of the haystack than the
        // backtracker.
        //
        // Now, if the haystack is really short already, then we allow the
        // backtracker to run. (This hasn't been litigated quantitatively with
        // benchmarks. Just a hunch.)
        if input.get_earliest() && input.haystack().len() > 128 {
            return None;
        }
        // If the backtracker is just going to return an error because the
        // haystack is too long, then obviously do not use it.
        if input.get_span().len() > engine.max_haystack_len() {
            return None;
        }
        Some(engine)
    }