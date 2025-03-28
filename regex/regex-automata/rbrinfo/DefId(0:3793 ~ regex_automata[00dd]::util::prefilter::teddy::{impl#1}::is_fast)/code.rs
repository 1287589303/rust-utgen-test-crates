fn is_fast(&self) -> bool {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            // Teddy is usually quite fast, but I have seen some cases where
            // a large number of literals can overwhelm it and make it not so
            // fast. We make an educated but conservative guess at a limit, at
            // which point, we're not so comfortable thinking Teddy is "fast."
            //
            // Well... this used to incorporate a "limit" on the *number*
            // of literals, but I have since changed it to a minimum on the
            // *smallest* literal. Namely, when there is a very small literal
            // (1 or 2 bytes), it is far more likely that it leads to a higher
            // false positive rate. (Although, of course, not always. For
            // example, 'zq' is likely to have a very low false positive rate.)
            // But when we have 3 bytes, we have a really good chance of being
            // quite discriminatory and thus fast.
            //
            // We may still want to add some kind of limit on the number of
            // literals here, but keep in mind that Teddy already has its own
            // somewhat small limit (64 at time of writing). The main issue
            // here is that if 'is_fast' is false, it opens the door for the
            // reverse inner optimization to kick in. We really only want to
            // resort to the reverse inner optimization if we absolutely must.
            self.minimum_len >= 3
        }
    }