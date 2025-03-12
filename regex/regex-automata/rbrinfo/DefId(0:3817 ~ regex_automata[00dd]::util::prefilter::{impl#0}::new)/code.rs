pub fn new<B: AsRef<[u8]>>(
        kind: MatchKind,
        needles: &[B],
    ) -> Option<Prefilter> {
        Choice::new(kind, needles).and_then(|choice| {
            let max_needle_len =
                needles.iter().map(|b| b.as_ref().len()).max().unwrap_or(0);
            Prefilter::from_choice(choice, max_needle_len)
        })
    }