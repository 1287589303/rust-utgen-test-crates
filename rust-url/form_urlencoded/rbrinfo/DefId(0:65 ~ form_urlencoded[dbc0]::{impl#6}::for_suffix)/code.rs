pub fn for_suffix(mut target: T, start_position: usize) -> Self {
        if target.as_mut_string().len() < start_position {
            panic!(
                "invalid length {} for target of length {}",
                start_position,
                target.as_mut_string().len()
            );
        }

        Serializer {
            target: Some(target),
            start_position,
            encoding: None,
        }
    }