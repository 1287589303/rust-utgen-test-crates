fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(key_must_be_a_string())
    }