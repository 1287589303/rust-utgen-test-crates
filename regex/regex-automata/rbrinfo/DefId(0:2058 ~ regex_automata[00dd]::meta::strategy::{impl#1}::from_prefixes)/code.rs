fn from_prefixes(
        info: &RegexInfo,
        prefixes: &literal::Seq,
    ) -> Option<Arc<dyn Strategy>> {
        let kind = info.config().get_match_kind();
        // Check to see if our prefixes are exact, which means we might be
        // able to bypass the regex engine entirely and just rely on literal
        // searches.
        if !prefixes.is_exact() {
            return None;
        }
        // We also require that we have a single regex pattern. Namely,
        // we reuse the prefilter infrastructure to implement search and
        // prefilters only report spans. Prefilters don't know about pattern
        // IDs. The multi-regex case isn't a lost cause, we might still use
        // Aho-Corasick and we might still just use a regular prefilter, but
        // that's done below.
        if info.pattern_len() != 1 {
            return None;
        }
        // We can't have any capture groups either. The literal engines don't
        // know how to deal with things like '(foo)(bar)'. In that case, a
        // prefilter will just be used and then the regex engine will resolve
        // the capture groups.
        if info.props()[0].explicit_captures_len() != 0 {
            return None;
        }
        // We also require that it has zero look-around assertions. Namely,
        // literal extraction treats look-around assertions as if they match
        // *every* empty string. But of course, that isn't true. So for
        // example, 'foo\bquux' never matches anything, but 'fooquux' is
        // extracted from that as an exact literal. Such cases should just run
        // the regex engine. 'fooquux' will be used as a normal prefilter, and
        // then the regex engine will try to look for an actual match.
        if !info.props()[0].look_set().is_empty() {
            return None;
        }
        // Finally, currently, our prefilters are all oriented around
        // leftmost-first match semantics, so don't try to use them if the
        // caller asked for anything else.
        if kind != MatchKind::LeftmostFirst {
            return None;
        }
        // The above seems like a lot of requirements to meet, but it applies
        // to a lot of cases. 'foo', '[abc][123]' and 'foo|bar|quux' all meet
        // the above criteria, for example.
        //
        // Note that this is effectively a latency optimization. If we didn't
        // do this, then the extracted literals would still get bundled into
        // a prefilter, and every regex engine capable of running unanchored
        // searches supports prefilters. So this optimization merely sidesteps
        // having to run the regex engine at all to confirm the match. Thus, it
        // decreases the latency of a match.

        // OK because we know the set is exact and thus finite.
        let prefixes = prefixes.literals().unwrap();
        debug!(
            "trying to bypass regex engine by creating \
             prefilter from {} literals: {:?}",
            prefixes.len(),
            prefixes,
        );
        let choice = match prefilter::Choice::new(kind, prefixes) {
            Some(choice) => choice,
            None => {
                debug!(
                    "regex bypass failed because no prefilter could be built"
                );
                return None;
            }
        };
        let strat: Arc<dyn Strategy> = match choice {
            prefilter::Choice::Memchr(pre) => Pre::new(pre),
            prefilter::Choice::Memchr2(pre) => Pre::new(pre),
            prefilter::Choice::Memchr3(pre) => Pre::new(pre),
            prefilter::Choice::Memmem(pre) => Pre::new(pre),
            prefilter::Choice::Teddy(pre) => Pre::new(pre),
            prefilter::Choice::ByteSet(pre) => Pre::new(pre),
            prefilter::Choice::AhoCorasick(pre) => Pre::new(pre),
        };
        Some(strat)
    }