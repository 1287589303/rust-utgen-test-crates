fn c_unicode_class(
        &self,
        cls: &hir::ClassUnicode,
    ) -> Result<ThompsonRef, BuildError> {
        // If all we have are ASCII ranges wrapped in a Unicode package, then
        // there is zero reason to bring out the big guns. We can fit all ASCII
        // ranges within a single sparse state.
        if cls.is_ascii() {
            let end = self.add_empty()?;
            let mut trans = Vec::with_capacity(cls.ranges().len());
            for r in cls.iter() {
                // The unwraps below are OK because we've verified that this
                // class only contains ASCII codepoints.
                trans.push(Transition {
                    // FIXME(1.59): use the 'TryFrom<char> for u8' impl.
                    start: u8::try_from(u32::from(r.start())).unwrap(),
                    end: u8::try_from(u32::from(r.end())).unwrap(),
                    next: end,
                });
            }
            Ok(ThompsonRef { start: self.add_sparse(trans)?, end })
        } else if self.is_reverse() {
            if !self.config.get_shrink() {
                // When we don't want to spend the extra time shrinking, we
                // compile the UTF-8 automaton in reverse using something like
                // the "naive" approach, but will attempt to re-use common
                // suffixes.
                self.c_unicode_class_reverse_with_suffix(cls)
            } else {
                // When we want to shrink our NFA for reverse UTF-8 automata,
                // we cannot feed UTF-8 sequences directly to the UTF-8
                // compiler, since the UTF-8 compiler requires all sequences
                // to be lexicographically sorted. Instead, we organize our
                // sequences into a range trie, which can then output our
                // sequences in the correct order. Unfortunately, building the
                // range trie is fairly expensive (but not nearly as expensive
                // as building a DFA). Hence the reason why the 'shrink' option
                // exists, so that this path can be toggled off. For example,
                // we might want to turn this off if we know we won't be
                // compiling a DFA.
                let mut trie = self.trie_state.borrow_mut();
                trie.clear();

                for rng in cls.iter() {
                    for mut seq in Utf8Sequences::new(rng.start(), rng.end()) {
                        seq.reverse();
                        trie.insert(seq.as_slice());
                    }
                }
                let mut builder = self.builder.borrow_mut();
                let mut utf8_state = self.utf8_state.borrow_mut();
                let mut utf8c =
                    Utf8Compiler::new(&mut *builder, &mut *utf8_state)?;
                trie.iter(|seq| {
                    utf8c.add(&seq)?;
                    Ok(())
                })?;
                utf8c.finish()
            }
        } else {
            // In the forward direction, we always shrink our UTF-8 automata
            // because we can stream it right into the UTF-8 compiler. There
            // is almost no downside (in either memory or time) to using this
            // approach.
            let mut builder = self.builder.borrow_mut();
            let mut utf8_state = self.utf8_state.borrow_mut();
            let mut utf8c =
                Utf8Compiler::new(&mut *builder, &mut *utf8_state)?;
            for rng in cls.iter() {
                for seq in Utf8Sequences::new(rng.start(), rng.end()) {
                    utf8c.add(seq.as_slice())?;
                }
            }
            utf8c.finish()
        }

        // For reference, the code below is the "naive" version of compiling a
        // UTF-8 automaton. It is deliciously simple (and works for both the
        // forward and reverse cases), but will unfortunately produce very
        // large NFAs. When compiling a forward automaton, the size difference
        // can sometimes be an order of magnitude. For example, the '\w' regex
        // will generate about ~3000 NFA states using the naive approach below,
        // but only 283 states when using the approach above. This is because
        // the approach above actually compiles a *minimal* (or near minimal,
        // because of the bounded hashmap for reusing equivalent states) UTF-8
        // automaton.
        //
        // The code below is kept as a reference point in order to make it
        // easier to understand the higher level goal here. Although, it will
        // almost certainly bit-rot, so keep that in mind. Also, if you try to
        // use it, some of the tests in this module will fail because they look
        // for terser byte code produce by the more optimized handling above.
        // But the integration test suite should still pass.
        //
        // One good example of the substantial difference this can make is to
        // compare and contrast performance of the Pike VM when the code below
        // is active vs the code above. Here's an example to try:
        //
        //   regex-cli find match pikevm -b -p '(?m)^\w{20}' non-ascii-file
        //
        // With Unicode classes generated below, this search takes about 45s on
        // my machine. But with the compressed version above, the search takes
        // only around 1.4s. The NFA is also 20% smaller. This is in part due
        // to the compression, but also because of the utilization of 'sparse'
        // NFA states. They lead to much less state shuffling during the NFA
        // search.
        /*
        let it = cls
            .iter()
            .flat_map(|rng| Utf8Sequences::new(rng.start(), rng.end()))
            .map(|seq| {
                let it = seq
                    .as_slice()
                    .iter()
                    .map(|rng| self.c_range(rng.start, rng.end));
                self.c_concat(it)
            });
        self.c_alt_iter(it)
        */
    }