pub fn len(&self) -> usize {
        let diff = 1 + u32::from(self.end) - u32::from(self.start);
        // This is likely to panic in 16-bit targets since a usize can only fit
        // 2^16. It's not clear what to do here, other than to return an error
        // when building a Unicode class that contains a range whose length
        // overflows usize. (Which, to be honest, is probably quite common on
        // 16-bit targets. For example, this would imply that '.' and '\p{any}'
        // would be impossible to build.)
        usize::try_from(diff).expect("char class len fits in usize")
    }