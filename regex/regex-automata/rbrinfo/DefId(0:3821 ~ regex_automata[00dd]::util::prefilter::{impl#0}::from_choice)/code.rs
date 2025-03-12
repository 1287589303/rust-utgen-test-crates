fn from_choice(
        choice: Choice,
        max_needle_len: usize,
    ) -> Option<Prefilter> {
        #[cfg(not(feature = "alloc"))]
        {
            None
        }
        #[cfg(feature = "alloc")]
        {
            let pre: Arc<dyn PrefilterI> = match choice {
                Choice::Memchr(p) => Arc::new(p),
                Choice::Memchr2(p) => Arc::new(p),
                Choice::Memchr3(p) => Arc::new(p),
                Choice::Memmem(p) => Arc::new(p),
                Choice::Teddy(p) => Arc::new(p),
                Choice::ByteSet(p) => Arc::new(p),
                Choice::AhoCorasick(p) => Arc::new(p),
            };
            let is_fast = pre.is_fast();
            Some(Prefilter { pre, is_fast, max_needle_len })
        }
    }