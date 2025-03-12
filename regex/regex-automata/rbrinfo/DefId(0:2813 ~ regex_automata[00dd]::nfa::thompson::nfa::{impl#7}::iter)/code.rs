pub(crate) fn iter(&self) -> impl Iterator<Item = Transition> + '_ {
        use crate::util::int::Usize;
        self.transitions
            .iter()
            .enumerate()
            .filter(|&(_, &sid)| sid != StateID::ZERO)
            .map(|(byte, &next)| Transition {
                start: byte.as_u8(),
                end: byte.as_u8(),
                next,
            })
    }