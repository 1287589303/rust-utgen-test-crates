fn parts1(r1: SplitRange) -> Split {
        // This value doesn't matter since it is never accessed.
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split { partitions: [r1, nada, nada], len: 1 }
    }