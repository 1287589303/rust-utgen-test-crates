fn as_accel_tys(&self) -> [AccelTy; 2] {
        assert_eq!(ACCEL_CAP, 8);
        // These unwraps are OK since ACCEL_CAP is set to 8.
        let first =
            AccelTy::from_ne_bytes(self.bytes[0..4].try_into().unwrap());
        let second =
            AccelTy::from_ne_bytes(self.bytes[4..8].try_into().unwrap());
        [first, second]
    }