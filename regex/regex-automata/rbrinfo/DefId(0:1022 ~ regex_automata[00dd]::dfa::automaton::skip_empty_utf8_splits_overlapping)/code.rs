fn skip_empty_utf8_splits_overlapping<F>(
    input: &Input<'_>,
    state: &mut OverlappingState,
    mut search: F,
) -> Result<(), MatchError>
where
    F: FnMut(&Input<'_>, &mut OverlappingState) -> Result<(), MatchError>,
{
    // Note that this routine works for forwards and reverse searches
    // even though there's no code here to handle those cases. That's
    // because overlapping searches drive themselves to completion via
    // `OverlappingState`. So all we have to do is push it until no matches are
    // found.

    let mut hm = match state.get_match() {
        None => return Ok(()),
        Some(hm) => hm,
    };
    if input.get_anchored().is_anchored() {
        if !input.is_char_boundary(hm.offset()) {
            state.mat = None;
        }
        return Ok(());
    }
    while !input.is_char_boundary(hm.offset()) {
        search(input, state)?;
        hm = match state.get_match() {
            None => return Ok(()),
            Some(hm) => hm,
        };
    }
    Ok(())
}