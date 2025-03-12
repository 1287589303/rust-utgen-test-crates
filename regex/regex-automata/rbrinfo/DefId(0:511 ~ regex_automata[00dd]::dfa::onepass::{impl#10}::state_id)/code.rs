fn state_id(&self) -> StateID {
        // OK because a Transition has a valid StateID in its upper bits by
        // construction. The cast to usize is also correct, even on 16-bit
        // targets because, again, we know the upper bits is a valid StateID,
        // which can never overflow usize on any supported target.
        StateID::new_unchecked(
            (self.0 >> Transition::STATE_ID_SHIFT).as_usize(),
        )
    }