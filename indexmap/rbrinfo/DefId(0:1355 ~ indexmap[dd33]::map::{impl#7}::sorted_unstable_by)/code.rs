pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        IntoIter::new(entries)
    }