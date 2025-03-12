fn optimize_by_preference(&mut self, prefix: bool) {
        let origlen = match self.len() {
            None => return,
            Some(len) => len,
        };
        // Just give up now if our sequence contains an empty string.
        if self.min_literal_len().map_or(false, |len| len == 0) {
            // We squash the sequence so that nobody else gets any bright
            // ideas to try and use it. An empty string implies a match at
            // every position. A prefilter cannot help you here.
            self.make_infinite();
            return;
        }
        // Make sure we start with the smallest sequence possible. We use a
        // special version of preference minimization that retains exactness.
        // This is legal because optimization is only expected to occur once
        // extraction is complete.
        if prefix {
            if let Some(ref mut lits) = self.literals {
                PreferenceTrie::minimize(lits, true);
            }
        }

        // Look for a common prefix (or suffix). If we found one of those and
        // it's long enough, then it's a good bet that it will be our fastest
        // possible prefilter since single-substring search is so fast.
        let fix = if prefix {
            self.longest_common_prefix()
        } else {
            self.longest_common_suffix()
        };
        if let Some(fix) = fix {
            // As a special case, if we have a common prefix and the leading
            // byte of that prefix is one that we think probably occurs rarely,
            // then strip everything down to just that single byte. This should
            // promote the use of memchr.
            //
            // ... we only do this though if our sequence has more than one
            // literal. Otherwise, we'd rather just stick with a single literal
            // scan. That is, using memchr is probably better than looking
            // for 2 or more literals, but probably not as good as a straight
            // memmem search.
            //
            // ... and also only do this when the prefix is short and probably
            // not too discriminatory anyway. If it's longer, then it's
            // probably quite discriminatory and thus is likely to have a low
            // false positive rate.
            if prefix
                && origlen > 1
                && fix.len() >= 1
                && fix.len() <= 3
                && rank(fix[0]) < 200
            {
                self.keep_first_bytes(1);
                self.dedup();
                return;
            }
            // We only strip down to the common prefix/suffix if we think
            // the existing set of literals isn't great, or if the common
            // prefix/suffix is expected to be particularly discriminatory.
            let isfast =
                self.is_exact() && self.len().map_or(false, |len| len <= 16);
            let usefix = fix.len() > 4 || (fix.len() > 1 && !isfast);
            if usefix {
                // If we keep exactly the number of bytes equal to the length
                // of the prefix (or suffix), then by the definition of a
                // prefix, every literal in the sequence will be equivalent.
                // Thus, 'dedup' will leave us with one literal.
                //
                // We do it this way to avoid an alloc, but also to make sure
                // the exactness of literals is kept (or not).
                if prefix {
                    self.keep_first_bytes(fix.len());
                } else {
                    self.keep_last_bytes(fix.len());
                }
                self.dedup();
                assert_eq!(Some(1), self.len());
                // We still fall through here. In particular, we want our
                // longest common prefix to be subject to the poison check.
            }
        }
        // If we have an exact sequence, we *probably* just want to keep it
        // as-is. But there are some cases where we don't. So we save a copy of
        // the exact sequence now, and then try to do some more optimizations
        // below. If those don't work out, we go back to this exact sequence.
        //
        // The specific motivation for this is that we sometimes wind up with
        // an exact sequence with a hefty number of literals. Say, 100. If we
        // stuck with that, it would be too big for Teddy and would result in
        // using Aho-Corasick. Which is fine... but the lazy DFA is plenty
        // suitable in such cases. The real issue is that we will wind up not
        // using a fast prefilter at all. So in cases like this, even though
        // we have an exact sequence, it would be better to try and shrink the
        // sequence (which we do below) and use it as a prefilter that can
        // produce false positive matches.
        //
        // But if the shrinking below results in a sequence that "sucks," then
        // we don't want to use that because we already have an exact sequence
        // in hand.
        let exact: Option<Seq> =
            if self.is_exact() { Some(self.clone()) } else { None };
        // Now we attempt to shorten the sequence. The idea here is that we
        // don't want to look for too many literals, but we want to shorten
        // our sequence enough to improve our odds of using better algorithms
        // downstream (such as Teddy).
        //
        // The pair of numbers in this list corresponds to the maximal prefix
        // (in bytes) to keep for all literals and the length of the sequence
        // at which to do it.
        //
        // So for example, the pair (3, 500) would mean, "if we have more than
        // 500 literals in our sequence, then truncate all of our literals
        // such that they are at most 3 bytes in length and the minimize the
        // sequence."
        const ATTEMPTS: [(usize, usize); 5] =
            [(5, 10), (4, 10), (3, 64), (2, 64), (1, 10)];
        for (keep, limit) in ATTEMPTS {
            let len = match self.len() {
                None => break,
                Some(len) => len,
            };
            if len <= limit {
                break;
            }
            if prefix {
                self.keep_first_bytes(keep);
            } else {
                self.keep_last_bytes(keep);
            }
            if prefix {
                if let Some(ref mut lits) = self.literals {
                    PreferenceTrie::minimize(lits, true);
                }
            }
        }
        // Check for a poison literal. A poison literal is one that is short
        // and is believed to have a very high match count. These poisons
        // generally lead to a prefilter with a very high false positive rate,
        // and thus overall worse performance.
        //
        // We do this last because we could have gone from a non-poisonous
        // sequence to a poisonous one. Perhaps we should add some code to
        // prevent such transitions in the first place, but then again, we
        // likely only made the transition in the first place if the sequence
        // was itself huge. And huge sequences are themselves poisonous. So...
        if let Some(lits) = self.literals() {
            if lits.iter().any(|lit| lit.is_poisonous()) {
                self.make_infinite();
            }
        }
        // OK, if we had an exact sequence before attempting more optimizations
        // above and our post-optimized sequence sucks for some reason or
        // another, then we go back to the exact sequence.
        if let Some(exact) = exact {
            // If optimizing resulted in dropping our literals, then certainly
            // backup and use the exact sequence that we had.
            if !self.is_finite() {
                *self = exact;
                return;
            }
            // If our optimized sequence contains a short literal, then it's
            // *probably* not so great. So throw it away and revert to the
            // exact sequence.
            if self.min_literal_len().map_or(true, |len| len <= 2) {
                *self = exact;
                return;
            }
            // Finally, if our optimized sequence is "big" (i.e., can't use
            // Teddy), then also don't use it and rely on the exact sequence.
            if self.len().map_or(true, |len| len > 64) {
                *self = exact;
                return;
            }
        }
    }