pub fn new<I, B>(it: I) -> Seq
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        it.into_iter().map(|b| Literal::exact(b.as_ref())).collect()
    }