pub fn search(&self, input: &Input<'_>) -> Option<Match> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        let mut guard = self.pool.get();
        let result = self.imp.strat.search(&mut guard, input);
        // We do this dance with the guard and explicitly put it back in the
        // pool because it seems to result in better codegen. If we let the
        // guard's Drop impl put it back in the pool, then functions like
        // ptr::drop_in_place get called and they *don't* get inlined. This
        // isn't usually a big deal, but in latency sensitive benchmarks the
        // extra function call can matter.
        //
        // I used `rebar measure -f '^grep/every-line$' -e meta` to measure
        // the effects here.
        //
        // Note that this doesn't eliminate the latency effects of using the
        // pool. There is still some (minor) cost for the "thread owner" of the
        // pool. (i.e., The thread that first calls a regex search routine.)
        // However, for other threads using the regex, the pool access can be
        // quite expensive as it goes through a mutex. Callers can avoid this
        // by either cloning the Regex (which creates a distinct copy of the
        // pool), or callers can use the lower level APIs that accept a 'Cache'
        // directly and do their own handling.
        PoolGuard::put(guard);
        result
    }