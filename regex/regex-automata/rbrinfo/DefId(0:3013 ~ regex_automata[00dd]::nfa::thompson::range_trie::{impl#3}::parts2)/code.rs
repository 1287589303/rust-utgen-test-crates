fn parts2(r1: SplitRange, r2: SplitRange) -> Split {
        // This value doesn't matter since it is never accessed.
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split { partitions: [r1, r2, nada], len: 2 }
    }