fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq {
        if self.class_over_limit_bytes(cls) {
            return Seq::infinite();
        }
        let mut seq = Seq::empty();
        for r in cls.iter() {
            for b in r.start()..=r.end() {
                seq.push(Literal::from(b));
            }
        }
        self.enforce_literal_len(&mut seq);
        seq
    }