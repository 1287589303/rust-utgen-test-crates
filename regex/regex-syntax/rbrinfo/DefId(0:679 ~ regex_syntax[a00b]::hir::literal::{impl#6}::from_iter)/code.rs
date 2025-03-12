fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq {
        let mut seq = Seq::empty();
        for literal in it {
            seq.push(literal);
        }
        seq
    }