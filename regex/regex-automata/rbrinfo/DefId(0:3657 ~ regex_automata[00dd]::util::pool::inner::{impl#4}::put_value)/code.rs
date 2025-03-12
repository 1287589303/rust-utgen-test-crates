fn put_value(&self, value: Box<T>) {
            let caller = THREAD_ID.with(|id| *id);
            let stack_id = caller % self.stacks.len();
            // As with trying to pop a value from this thread's stack, we
            // merely attempt to get access to push this value back on the
            // stack. If there's too much contention, we just give up and throw
            // the value away.
            //
            // Interestingly, in ad hoc benchmarking, it is beneficial to
            // attempt to push the value back more than once, unlike when
            // popping the value. I don't have a good theory for why this is.
            // I guess if we drop too many values then that winds up forcing
            // the pop operation to create new fresh values and thus leads to
            // less reuse. There's definitely a balancing act here.
            for _ in 0..10 {
                let mut stack = match self.stacks[stack_id].0.try_lock() {
                    Err(_) => continue,
                    Ok(stack) => stack,
                };
                stack.push(value);
                return;
            }
        }