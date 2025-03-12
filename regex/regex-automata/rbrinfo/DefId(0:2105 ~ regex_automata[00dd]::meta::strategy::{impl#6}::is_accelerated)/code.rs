fn is_accelerated(&self) -> bool {
        // Since this is anchored at the end, a reverse anchored search is
        // almost certainly guaranteed to result in a much faster search than
        // a standard forward search.
        true
    }