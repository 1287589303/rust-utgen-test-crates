pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {
        wire::check_slice_len(slice, 256, "byte class map")?;
        let mut classes = ByteClasses::empty();
        for (b, &class) in slice[..256].iter().enumerate() {
            classes.set(u8::try_from(b).unwrap(), class);
        }
        // We specifically don't use 'classes.iter()' here because that
        // iterator depends on 'classes.alphabet_len()' being correct. But that
        // is precisely the thing we're trying to verify below!
        for &b in classes.0.iter() {
            if usize::from(b) >= classes.alphabet_len() {
                return Err(DeserializeError::generic(
                    "found equivalence class greater than alphabet len",
                ));
            }
        }
        Ok((classes, 256))
    }