pub(crate) fn skip_splits_fwd<T, F>(
    input: &Input<'_>,
    init_value: T,
    match_offset: usize,
    find: F,
) -> Result<Option<T>, MatchError>
where
    F: FnMut(&Input<'_>) -> Result<Option<(T, usize)>, MatchError>,
{
    skip_splits(true, input, init_value, match_offset, find)
}