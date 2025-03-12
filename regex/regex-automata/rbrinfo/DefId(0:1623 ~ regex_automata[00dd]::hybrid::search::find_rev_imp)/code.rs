fn find_rev_imp(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
    earliest: bool,
) -> Result<Option<HalfMatch>, MatchError> {
    let mut mat = None;
    let mut sid = init_rev(dfa, cache, input)?;
    // In reverse search, the loop below can't handle the case of searching an
    // empty slice. Ideally we could write something congruent to the forward
    // search, i.e., 'while at >= start', but 'start' might be 0. Since we use
    // an unsigned offset, 'at >= 0' is trivially always true. We could avoid
    // this extra case handling by using a signed offset, but Rust makes it
    // annoying to do. So... We just handle the empty case separately.
    if input.start() == input.end() {
        eoi_rev(dfa, cache, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }

    let mut at = input.end() - 1;
    macro_rules! next_unchecked {
        ($sid:expr, $at:expr) => {{
            let byte = *input.haystack().get_unchecked($at);
            dfa.next_state_untagged_unchecked(cache, $sid, byte)
        }};
    }
    cache.search_start(at);
    loop {
        if sid.is_tagged() {
            cache.search_update(at);
            sid = dfa
                .next_state(cache, sid, input.haystack()[at])
                .map_err(|_| gave_up(at))?;
        } else {
            // SAFETY: See comments in 'find_fwd' for a safety argument.
            //
            // PERF: The comments in 'find_fwd' also provide a justification
            // from a performance perspective as to 1) why we elide bounds
            // checks and 2) why we do a specialized version of unrolling
            // below. The reverse search does have a slightly different
            // consideration in that most reverse searches tend to be
            // anchored and on shorter haystacks. However, this still makes a
            // difference. Take this command for example:
            //
            //     regex-cli find match hybrid -p '(?m)^.+$' -UBb bigfile
            //
            // (Notice that we use 'find hybrid regex', not 'find hybrid dfa'
            // like in the justification for the forward direction. The 'regex'
            // sub-command will find start-of-match and thus run the reverse
            // direction.)
            //
            // Without unrolling below, the above command takes around 3.76s.
            // But with the unrolling below, we get down to 2.55s. If we keep
            // the unrolling but add in bounds checks, then we get 2.86s.
            //
            // NOTE: I used 'OpenSubtitles2018.raw.sample.en' for 'bigfile'.
            let mut prev_sid = sid;
            while at >= input.start() {
                prev_sid = unsafe { next_unchecked!(sid, at) };
                if prev_sid.is_tagged()
                    || at <= input.start().saturating_add(3)
                {
                    core::mem::swap(&mut prev_sid, &mut sid);
                    break;
                }
                at -= 1;

                sid = unsafe { next_unchecked!(prev_sid, at) };
                if sid.is_tagged() {
                    break;
                }
                at -= 1;

                prev_sid = unsafe { next_unchecked!(sid, at) };
                if prev_sid.is_tagged() {
                    core::mem::swap(&mut prev_sid, &mut sid);
                    break;
                }
                at -= 1;

                sid = unsafe { next_unchecked!(prev_sid, at) };
                if sid.is_tagged() {
                    break;
                }
                at -= 1;
            }
            // If we quit out of the code above with an unknown state ID at
            // any point, then we need to re-compute that transition using
            // 'next_state', which will do NFA powerset construction for us.
            if sid.is_unknown() {
                cache.search_update(at);
                sid = dfa
                    .next_state(cache, prev_sid, input.haystack()[at])
                    .map_err(|_| gave_up(at))?;
            }
        }
        if sid.is_tagged() {
            if sid.is_start() {
                // do nothing
            } else if sid.is_match() {
                let pattern = dfa.match_pattern(cache, sid, 0);
                // Since reverse searches report the beginning of a match
                // and the beginning is inclusive (not exclusive like the
                // end of a match), we add 1 to make it inclusive.
                mat = Some(HalfMatch::new(pattern, at + 1));
                if earliest {
                    cache.search_finish(at);
                    return Ok(mat);
                }
            } else if sid.is_dead() {
                cache.search_finish(at);
                return Ok(mat);
            } else if sid.is_quit() {
                cache.search_finish(at);
                return Err(MatchError::quit(input.haystack()[at], at));
            } else {
                debug_assert!(sid.is_unknown());
                unreachable!("sid being unknown is a bug");
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
    }
    cache.search_finish(input.start());
    eoi_rev(dfa, cache, input, &mut sid, &mut mat)?;
    Ok(mat)
}