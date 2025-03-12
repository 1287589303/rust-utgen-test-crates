fn find(&self, range: Utf8Range) -> usize {
        /// Returns the position `i` at which `pred(xs[i])` first returns true
        /// such that for all `j >= i`, `pred(xs[j]) == true`. If `pred` never
        /// returns true, then `xs.len()` is returned.
        ///
        /// We roll our own binary search because it doesn't seem like the
        /// standard library's binary search can be used here. Namely, if
        /// there is an overlapping range, then we want to find the first such
        /// occurrence, but there may be many. Or at least, it's not quite
        /// clear to me how to do it.
        fn binary_search<T, F>(xs: &[T], mut pred: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            let (mut left, mut right) = (0, xs.len());
            while left < right {
                // Overflow is impossible because xs.len() <= 256.
                let mid = (left + right) / 2;
                if pred(&xs[mid]) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }

        // Benchmarks suggest that binary search is just a bit faster than
        // straight linear search. Specifically when using the debug tool:
        //
        //   hyperfine "regex-cli debug thompson -qr --captures none '\w{90} ecurB'"
        binary_search(&self.transitions, |t| range.start <= t.range.end)
    }