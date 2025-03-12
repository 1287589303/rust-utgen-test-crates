fn union(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {
        if seq1.max_union_len(seq2).map_or(false, |len| len > self.limit_total)
        {
            // We try to trim our literal sequences to see if we can make
            // room for more literals. The idea is that we'd rather trim down
            // literals already in our sequence if it means we can add a few
            // more and retain a finite sequence. Otherwise, we'll union with
            // an infinite sequence and that infects everything and effectively
            // stops literal extraction in its tracks.
            //
            // We do we keep 4 bytes here? Well, it's a bit of an abstraction
            // leakage. Downstream, the literals may wind up getting fed to
            // the Teddy algorithm, which supports searching literals up to
            // length 4. So that's why we pick that number here. Arguably this
            // should be a tuneable parameter, but it seems a little tricky to
            // describe. And I'm still unsure if this is the right way to go
            // about culling literal sequences.
            match self.kind {
                ExtractKind::Prefix => {
                    seq1.keep_first_bytes(4);
                    seq2.keep_first_bytes(4);
                }
                ExtractKind::Suffix => {
                    seq1.keep_last_bytes(4);
                    seq2.keep_last_bytes(4);
                }
            }
            seq1.dedup();
            seq2.dedup();
            if seq1
                .max_union_len(seq2)
                .map_or(false, |len| len > self.limit_total)
            {
                seq2.make_infinite();
            }
        }
        seq1.union(seq2);
        assert!(seq1.len().map_or(true, |x| x <= self.limit_total));
        seq1
    }