pub(crate) fn iter_splits<'a>(
        splits: &'a [StateID],
        reverse: bool,
    ) -> impl Iterator<Item = StateID> + 'a {
        let mut it = splits.iter();
        core::iter::from_fn(move || {
            if reverse { it.next_back() } else { it.next() }.copied()
        })
    }