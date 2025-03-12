pub fn representatives<R: core::ops::RangeBounds<u8>>(
        &self,
        range: R,
    ) -> ByteClassRepresentatives<'_> {
        use core::ops::Bound;

        let cur_byte = match range.start_bound() {
            Bound::Included(&i) => usize::from(i),
            Bound::Excluded(&i) => usize::from(i).checked_add(1).unwrap(),
            Bound::Unbounded => 0,
        };
        let end_byte = match range.end_bound() {
            Bound::Included(&i) => {
                Some(usize::from(i).checked_add(1).unwrap())
            }
            Bound::Excluded(&i) => Some(usize::from(i)),
            Bound::Unbounded => None,
        };
        assert_ne!(
            cur_byte,
            usize::MAX,
            "start range must be less than usize::MAX",
        );
        ByteClassRepresentatives {
            classes: self,
            cur_byte,
            end_byte,
            last_class: None,
        }
    }