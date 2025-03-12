fn as_usize(self) -> usize {
        // OK because we require 32 or 64 bit targets. Therefore, every u32
        // necessarily fits into a usize.
        self as usize
    }