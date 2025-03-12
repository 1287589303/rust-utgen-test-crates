pub(super) fn new(create: F) -> Pool<T, F> {
            // FIXME: Now that we require 1.65+, Mutex::new is available as
            // const... So we can almost mark this function as const. But of
            // course, we're creating a Vec of stacks below (we didn't when I
            // originally wrote this code). It seems like the best way to work
            // around this would be to use a `[Stack; MAX_POOL_STACKS]` instead
            // of a `Vec<Stack>`. I refrained from making this change at time
            // of writing (2023/10/08) because I was making a lot of other
            // changes at the same time and wanted to do this more carefully.
            // Namely, because of the cache line optimization, that `[Stack;
            // MAX_POOL_STACKS]` would be quite big. It's unclear how bad (if
            // at all) that would be.
            //
            // Another choice would be to lazily allocate the stacks, but...
            // I'm not so sure about that. Seems like a fair bit of complexity?
            //
            // Maybe there's a simple solution I'm missing.
            //
            // ... OK, I tried to fix this. First, I did it by putting `stacks`
            // in an `UnsafeCell` and using a `Once` to lazily initialize it.
            // I benchmarked it and everything looked okay. I then made this
            // function `const` and thought I was just about done. But the
            // public pool type wraps its inner pool in a `Box` to keep its
            // size down. Blech.
            //
            // So then I thought that I could push the box down into this
            // type (and leave the non-std version unboxed) and use the same
            // `UnsafeCell` technique to lazily initialize it. This has the
            // downside of the `Once` now needing to get hit in the owner fast
            // path, but maybe that's OK? However, I then realized that we can
            // only lazily initialize `stacks`, `owner` and `owner_val`. The
            // `create` function needs to be put somewhere outside of the box.
            // So now the pool is a `Box`, `Once` and a function. Now we're
            // starting to defeat the point of boxing in the first place. So I
            // backed out that change too.
            //
            // Back to square one. I maybe we just don't make a pool's
            // constructor const and live with it. It's probably not a huge
            // deal.
            let mut stacks = Vec::with_capacity(MAX_POOL_STACKS);
            for _ in 0..stacks.capacity() {
                stacks.push(CacheLine(Mutex::new(vec![])));
            }
            let owner = AtomicUsize::new(THREAD_ID_UNOWNED);
            let owner_val = UnsafeCell::new(None); // init'd on first access
            Pool { create, stacks, owner, owner_val }
        }